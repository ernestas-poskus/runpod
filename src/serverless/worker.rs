//! In-container RunPod Serverless worker loop.

use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Error, Result};

#[cfg(feature = "tracing")]
const TRACING_TARGET: &str = "runpod_sdk::serverless::worker";

/// A job acquired by a RunPod Serverless worker.
#[derive(Debug, Clone, Deserialize)]
pub struct WorkerJob {
    /// RunPod request id.
    pub id: String,
    /// User-submitted payload from the endpoint request's `input` field.
    pub input: Value,
    /// Any additional fields RunPod includes with the job.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

/// Result returned to RunPod for a processed worker job.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerJobResult {
    /// Successful job output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Value>,
    /// Failed job error message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Ask RunPod to refresh the worker after returning this result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_pod: Option<bool>,
}

impl WorkerJobResult {
    /// Build a successful job result.
    pub fn output(output: impl Into<Value>) -> Self {
        Self {
            output: Some(output.into()),
            error: None,
            stop_pod: None,
        }
    }

    /// Build a failed job result.
    pub fn error(error: impl Into<String>) -> Self {
        Self {
            output: None,
            error: Some(error.into()),
            stop_pod: None,
        }
    }
}

/// Configuration for an in-container RunPod Serverless worker.
#[derive(Debug, Clone)]
pub struct WorkerConfig {
    /// Worker id; defaults to `RUNPOD_POD_ID`.
    pub worker_id: String,
    /// Job acquisition URL from `RUNPOD_WEBHOOK_GET_JOB`.
    pub get_job_url: String,
    /// Job result URL from `RUNPOD_WEBHOOK_POST_OUTPUT`.
    pub post_output_url: String,
    /// Optional heartbeat URL from `RUNPOD_WEBHOOK_PING`.
    pub ping_url: Option<String>,
    /// Authorization value sent to RunPod worker endpoints.
    pub api_key: Option<String>,
    /// Poll concurrency. Defaults to 1.
    pub concurrency: usize,
    /// Heartbeat interval. Defaults to `RUNPOD_PING_INTERVAL` milliseconds or 10s.
    pub ping_interval: Duration,
    /// Request timeout for job polling and result posts.
    pub request_timeout: Duration,
}

impl WorkerConfig {
    /// Build worker config from RunPod Serverless environment variables.
    pub fn from_env() -> Result<Self> {
        let worker_id = std::env::var("RUNPOD_POD_ID")
            .map_err(|_| Error::Job("RUNPOD_POD_ID is missing".to_string()))?;
        let get_job_url = std::env::var("RUNPOD_WEBHOOK_GET_JOB")
            .map_err(|_| Error::Job("RUNPOD_WEBHOOK_GET_JOB is missing".to_string()))?
            .replace("$ID", &worker_id)
            .replace("$RUNPOD_POD_ID", &worker_id);
        let post_output_url = std::env::var("RUNPOD_WEBHOOK_POST_OUTPUT")
            .map_err(|_| Error::Job("RUNPOD_WEBHOOK_POST_OUTPUT is missing".to_string()))?
            .replace("$RUNPOD_POD_ID", &worker_id);
        let ping_url = std::env::var("RUNPOD_WEBHOOK_PING").ok().map(|url| {
            url.replace("$ID", &worker_id)
                .replace("$RUNPOD_POD_ID", &worker_id)
        });
        let api_key = std::env::var("RUNPOD_AI_API_KEY").ok();
        let concurrency = std::env::var("RUNPOD_WORKER_CONCURRENCY")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(1);
        let ping_interval = std::env::var("RUNPOD_PING_INTERVAL")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_secs(10));
        let request_timeout = std::env::var("RUNPOD_WORKER_REQUEST_TIMEOUT_SECONDS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(600));

        Ok(Self {
            worker_id,
            get_job_url,
            post_output_url,
            ping_url,
            api_key,
            concurrency,
            ping_interval,
            request_timeout,
        })
    }
}

/// RunPod Serverless worker loop.
#[derive(Clone)]
pub struct ServerlessWorker {
    config: WorkerConfig,
    client: Client,
    active_jobs: Arc<Mutex<Vec<String>>>,
}

impl ServerlessWorker {
    /// Create a worker from explicit configuration.
    pub fn new(config: WorkerConfig) -> Result<Self> {
        let client = Client::builder().timeout(config.request_timeout).build()?;
        Ok(Self {
            config,
            client,
            active_jobs: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Create a worker from RunPod Serverless environment variables.
    pub fn from_env() -> Result<Self> {
        Self::new(WorkerConfig::from_env()?)
    }

    /// Run the worker loop forever, with up to [`WorkerConfig::concurrency`]
    /// jobs in flight at once.
    ///
    /// Spawns `concurrency` independent copies of the poll/process loop
    /// (`concurrency` defaults to 1, from `RUNPOD_WORKER_CONCURRENCY` via
    /// [`WorkerConfig::from_env`]) — each copy is still internally
    /// sequential (`take_job` → await `handler` to completion →
    /// `post_result` → repeat), but running `concurrency` of them side by
    /// side lets that many jobs actually overlap, which matters for any
    /// handler backed by a worker pool wider than one (e.g. multiple GPU
    /// inference slots). A single heartbeat task is still shared across
    /// all of them and reports every concurrently active job id (see
    /// `start_heartbeat`/`active_jobs`).
    ///
    /// `handler` needs `Send + Sync + 'static` (and `Fut: Send + 'static`)
    /// so it can be shared across the spawned tasks — virtually always
    /// true already for realistic async worker handlers running on a
    /// multi-threaded Tokio runtime.
    pub async fn run<H, Fut>(&self, handler: H) -> Result<()>
    where
        H: Fn(WorkerJob) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = std::result::Result<Value, String>> + Send + 'static,
    {
        self.start_heartbeat();

        let concurrency = self.config.concurrency.max(1);
        let handler = Arc::new(handler);

        let mut tasks = Vec::with_capacity(concurrency);
        for _ in 0..concurrency {
            let worker = self.clone();
            let handler = Arc::clone(&handler);
            tasks.push(tokio::spawn(async move {
                worker.run_loop(handler.as_ref()).await
            }));
        }

        for task in tasks {
            task.await
                .map_err(|error| Error::Job(format!("worker task panicked: {error}")))?;
        }
        Ok(())
    }

    /// The single-job-at-a-time poll/process loop `run` spawns
    /// `concurrency` copies of. Never returns under normal operation — see
    /// `run`'s doc comment.
    ///
    /// Prefetches the next job (`take_job(true)`, `job_in_progress=true` —
    /// RunPod's own sanctioned "I'm still busy, but tell me what's next"
    /// signal, the same one RunPod's official Python/Go workers use) while
    /// `handler` is still running for the current job, instead of only
    /// polling after `post_result` completes. Without this, a worker whose
    /// handler is backed by a single-threaded resource (e.g. one dedicated
    /// GPU inference thread — `concurrency` can't help there, since more
    /// copies of this loop would just queue behind the same resource) sits
    /// fully idle for a whole `take_job` HTTP round trip between every pair
    /// of jobs, even when the queue already has the next job waiting. This
    /// doesn't change `run`/`run_once`'s public contract — a caller driving
    /// `run_once` directly (as the existing test suite does) still gets the
    /// simple non-prefetching one-job-at-a-time behavior; only `run`'s
    /// internal loop benefits.
    async fn run_loop<H, Fut>(&self, handler: &H)
    where
        H: Fn(WorkerJob) -> Fut,
        Fut: Future<Output = std::result::Result<Value, String>>,
    {
        let mut prefetched: Option<WorkerJob> = None;
        loop {
            let job = match prefetched.take() {
                Some(job) => job,
                None => match self.take_job(false).await {
                    Ok(Some(job)) => job,
                    Ok(None) => {
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        continue;
                    }
                    Err(error) => {
                        #[cfg(feature = "tracing")]
                        tracing::warn!(target: TRACING_TARGET, error = %error, "failed to take RunPod job");
                        #[cfg(not(feature = "tracing"))]
                        let _ = &error;
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        continue;
                    }
                },
            };

            #[cfg(feature = "tracing")]
            tracing::info!(target: TRACING_TARGET, job_id = %job.id, "received RunPod worker job");

            self.mark_job_active(&job.id);
            let (handler_result, prefetch_result) =
                tokio::join!(handler(job.clone()), self.take_job(true));
            let result = match handler_result {
                Ok(output) => WorkerJobResult::output(output),
                Err(error) => WorkerJobResult::error(error),
            };
            match prefetch_result {
                Ok(next) => prefetched = next,
                Err(error) => {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(target: TRACING_TARGET, error = %error, "failed to prefetch next RunPod job");
                    #[cfg(not(feature = "tracing"))]
                    let _ = &error;
                }
            }
            let post_result = self.post_result(&job, &result, false).await;
            self.mark_job_inactive(&job.id);
            if let Err(error) = post_result {
                #[cfg(feature = "tracing")]
                tracing::warn!(target: TRACING_TARGET, error = %error, "failed to post RunPod job result");
                #[cfg(not(feature = "tracing"))]
                let _ = &error;
            }
        }
    }

    /// Poll for one job and process it if present.
    ///
    /// Returns `true` when a job was processed and `false` when RunPod returned
    /// no job for this worker.
    pub async fn run_once<H, Fut>(&self, handler: &H) -> Result<bool>
    where
        H: Fn(WorkerJob) -> Fut,
        Fut: Future<Output = std::result::Result<Value, String>>,
    {
        let Some(job) = self.take_job(false).await? else {
            return Ok(false);
        };

        #[cfg(feature = "tracing")]
        tracing::info!(target: TRACING_TARGET, job_id = %job.id, "received RunPod worker job");

        self.mark_job_active(&job.id);
        let result = match handler(job.clone()).await {
            Ok(output) => WorkerJobResult::output(output),
            Err(error) => WorkerJobResult::error(error),
        };
        let post_result = self.post_result(&job, &result, false).await;
        self.mark_job_inactive(&job.id);
        post_result?;
        Ok(true)
    }

    fn mark_job_active(&self, job_id: &str) {
        if let Ok(mut jobs) = self.active_jobs.lock() {
            jobs.push(job_id.to_string());
        }
    }

    fn mark_job_inactive(&self, job_id: &str) {
        if let Ok(mut jobs) = self.active_jobs.lock() {
            jobs.retain(|active| active != job_id);
        }
    }

    async fn take_job(&self, job_in_progress: bool) -> Result<Option<WorkerJob>> {
        let url = append_query(
            &self.config.get_job_url,
            "job_in_progress",
            if job_in_progress { "1" } else { "0" },
        );
        let mut request = self.client.get(url);
        if let Some(api_key) = &self.config.api_key {
            request = request.header("Authorization", api_key);
        }

        let response = request.send().await?;
        match response.status().as_u16() {
            204 | 400 => Ok(None),
            429 => {
                tokio::time::sleep(Duration::from_secs(5)).await;
                Ok(None)
            }
            _ => {
                let response = response.error_for_status()?;
                let job: WorkerJob = response.json().await?;
                Ok(Some(job))
            }
        }
    }

    async fn post_result(
        &self,
        job: &WorkerJob,
        result: &WorkerJobResult,
        is_stream: bool,
    ) -> Result<()> {
        let mut url = self.config.post_output_url.replace("$ID", &job.id);
        url = append_query(&url, "isStream", if is_stream { "true" } else { "false" });
        let body = serde_json::to_string(result)?;

        let mut request = self
            .client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("charset", "utf-8")
            .header("X-Request-ID", &job.id)
            .body(body);
        if let Some(api_key) = &self.config.api_key {
            request = request.header("Authorization", api_key);
        }

        let response = request.send().await?;
        if let Err(error) = response.error_for_status_ref() {
            #[cfg(feature = "tracing")]
            {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                tracing::warn!(
                    target: TRACING_TARGET,
                    job_id = %job.id,
                    %status,
                    body = %body,
                    "RunPod worker result post failed"
                );
            }

            return Err(error.into());
        }

        #[cfg(feature = "tracing")]
        tracing::info!(target: TRACING_TARGET, job_id = %job.id, "RunPod worker result sent");

        Ok(())
    }

    fn start_heartbeat(&self) {
        let Some(ping_url) = self.config.ping_url.clone() else {
            return;
        };
        let client = self.client.clone();
        let api_key = self.config.api_key.clone();
        let interval = self.config.ping_interval;
        let active_jobs = Arc::clone(&self.active_jobs);

        tokio::spawn(async move {
            loop {
                let mut request = client
                    .get(&ping_url)
                    .query(&[("runpod_version", env!("CARGO_PKG_VERSION"))]);
                let job_ids = active_jobs
                    .lock()
                    .map(|jobs| jobs.clone())
                    .unwrap_or_default();
                if !job_ids.is_empty() {
                    request = request.query(&[("job_id", job_ids.join(",").as_str())]);
                }
                if let Some(api_key) = &api_key {
                    request = request.header("Authorization", api_key);
                }

                if let Err(error) = request.send().await {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(target: TRACING_TARGET, error = %error, "RunPod heartbeat failed");
                    #[cfg(not(feature = "tracing"))]
                    let _ = &error;
                }

                tokio::time::sleep(interval).await;
            }
        });
    }
}

fn append_query(url: &str, key: &str, value: &str) -> String {
    let separator = if url.contains('?') { '&' } else { '?' };
    format!("{url}{separator}{key}={value}")
}
