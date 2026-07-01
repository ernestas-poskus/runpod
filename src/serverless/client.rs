//! Serverless endpoint runner for running serverless jobs

use std::sync::Arc;

use serde::Serialize;

use super::job::ServerlessJob;
use super::types::EndpointHealth;
use crate::{Result, RunpodClient};

#[cfg(feature = "tracing")]
const TRACING_TARGET: &str = "runpod_sdk::serverless";

/// Class for running jobs on a specific endpoint.
///
/// # Examples
///
/// ```no_run
/// use runpod_sdk::{RunpodClient, Result};
/// use runpod_sdk::serverless::ServerlessEndpoint;
/// use serde_json::json;
///
/// # async fn example() -> Result<()> {
/// let client = RunpodClient::from_env()?;
/// let endpoint = ServerlessEndpoint::new("YOUR_ENDPOINT_ID", client);
///
/// let job = endpoint.run(&json!({"prompt": "Hello, world!"}))?;
/// let output: serde_json::Value = job.await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct ServerlessEndpoint {
    endpoint_id: Arc<String>,
    client: RunpodClient,
}

impl ServerlessEndpoint {
    /// Creates a new Endpoint instance.
    ///
    /// # Arguments
    ///
    /// * `endpoint_id` - The unique identifier for the serverless endpoint
    /// * `client` - Reference to the RunpodClient
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::serverless::ServerlessEndpoint;
    /// # fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = ServerlessEndpoint::new("ENDPOINT_ID", client);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(endpoint_id: impl Into<String>, client: RunpodClient) -> Self {
        Self {
            endpoint_id: Arc::new(endpoint_id.into()),
            client,
        }
    }

    /// Returns the endpoint ID.
    pub fn endpoint_id(&self) -> &str {
        &self.endpoint_id
    }

    /// Runs a job on the endpoint.
    ///
    /// # Arguments
    ///
    /// * `input` - The input payload for the job
    ///
    /// # Returns
    ///
    /// Returns a Job instance that implements Future for retrieving results.
    /// The job submission happens when you first poll the Job (e.g., by awaiting it).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::serverless::ServerlessEndpoint;
    /// # use serde::{Deserialize, Serialize};
    /// # use serde_json::json;
    /// #
    /// # #[derive(Serialize)]
    /// # struct Input {
    /// #     prompt: String,
    /// # }
    /// #
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = ServerlessEndpoint::new("ENDPOINT_ID", client);
    ///
    /// let job = endpoint.run(&Input {
    ///     prompt: "Hello, World!".to_string()
    /// })?;
    ///
    /// let output: serde_json::Value = job.await?;
    /// println!("Job result: {:?}", output);
    /// # Ok(())
    /// # }
    /// ```
    pub fn run<I>(&self, input: &I) -> Result<ServerlessJob>
    where
        I: Serialize,
    {
        let input_value = serde_json::to_value(input)?;

        Ok(ServerlessJob::new(
            Arc::clone(&self.endpoint_id),
            input_value,
            self.client.clone(),
        ))
    }

    /// Runs a job and immediately waits for the result.
    ///
    /// This is a convenience method that runs a job and awaits its completion.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::serverless::ServerlessEndpoint;
    /// # use serde_json::json;
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = ServerlessEndpoint::new("ENDPOINT_ID", client);
    ///
    /// let output: serde_json::Value = endpoint.run_now(&json!({"prompt": "Hello"})).await?;
    /// println!("Result: {:?}", output);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run_now<I, O>(&self, input: &I) -> Result<O>
    where
        I: Serialize,
        O: serde::de::DeserializeOwned,
    {
        let job = self.run(input)?;
        let value = job.await?;
        Ok(serde_json::from_value(value)?)
    }

    /// Checks the health of the endpoint.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::serverless::ServerlessEndpoint;
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = ServerlessEndpoint::new("ENDPOINT_ID", client);
    ///
    /// let health = endpoint.health().await?;
    /// println!("Workers ready: {}", health.workers.ready);
    /// println!("Jobs in queue: {}", health.jobs.in_queue);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn health(&self) -> Result<EndpointHealth> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET,
            endpoint_id = %self.endpoint_id,
            "Checking endpoint health"
        );

        let path = format!("{}/health", self.endpoint_id);

        let response = self.client.get_api(&path).send().await?;
        let response = response.error_for_status()?;
        let health: EndpointHealth = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET,
            endpoint_id = %self.endpoint_id,
            workers_ready = health.workers.ready,
            jobs_in_queue = health.jobs.in_queue,
            "Endpoint health retrieved"
        );

        Ok(health)
    }

    /// Purges all jobs from the endpoint's queue.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, Result};
    /// # use runpod_sdk::serverless::ServerlessEndpoint;
    /// # async fn example() -> Result<()> {
    /// let client = RunpodClient::from_env()?;
    /// let endpoint = ServerlessEndpoint::new("ENDPOINT_ID", client);
    ///
    /// endpoint.purge_queue().await?;
    /// println!("Queue purged");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn purge_queue(&self) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: TRACING_TARGET,
            endpoint_id = %self.endpoint_id,
            "Purging endpoint queue"
        );

        let path = format!("{}/purge-queue", self.endpoint_id);

        let response = self.client.post_api(&path).send().await?;
        response.error_for_status()?;

        #[cfg(feature = "tracing")]
        tracing::info!(
            target: TRACING_TARGET,
            endpoint_id = %self.endpoint_id,
            "Endpoint queue purged successfully"
        );

        Ok(())
    }
}

impl std::fmt::Debug for ServerlessEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Endpoint")
            .field("endpoint_id", &self.endpoint_id)
            .finish()
    }
}
