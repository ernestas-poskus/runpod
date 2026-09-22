#![cfg(feature = "serverless")]

use std::future;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use runpod_sdk::serverless::{ServerlessWorker, WorkerConfig};
use serde_json::json;
use wiremock::matchers::{body_string_contains, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn worker_takes_job_and_posts_output() {
    let server = MockServer::start().await;
    let config = WorkerConfig {
        worker_id: "worker-1".to_string(),
        get_job_url: format!("{}/job-take/worker-1?token=abc", server.uri()),
        post_output_url: format!("{}/job-done/$ID?token=abc", server.uri()),
        ping_url: None,
        api_key: Some("worker-key".to_string()),
        concurrency: 1,
        ping_interval: Duration::from_secs(10),
        request_timeout: Duration::from_secs(5),
    };
    let worker = ServerlessWorker::new(config).unwrap();

    Mock::given(method("GET"))
        .and(path("/job-take/worker-1"))
        .and(query_param("token", "abc"))
        .and(query_param("job_in_progress", "0"))
        .and(header("authorization", "worker-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "job-1",
            "input": {"text": "hello"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/job-done/job-1"))
        .and(query_param("token", "abc"))
        .and(query_param("isStream", "false"))
        .and(header("authorization", "worker-key"))
        .and(header("x-request-id", "job-1"))
        .and(body_string_contains("\"output\":{\"ok\":true}"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let processed = worker
        .run_once(&|job| async move {
            assert_eq!(job.id, "job-1");
            assert_eq!(job.input["text"], "hello");
            Ok(json!({"ok": true}))
        })
        .await
        .unwrap();

    assert!(processed);
}

#[tokio::test]
async fn worker_preserves_job_id_placeholder_until_result_post() {
    let server = MockServer::start().await;
    let config = WorkerConfig {
        worker_id: "worker-1".to_string(),
        get_job_url: format!("{}/job-take/worker-1?token=abc", server.uri()),
        post_output_url: format!("{}/job-done/worker-1/$ID?token=abc", server.uri()),
        ping_url: None,
        api_key: Some("worker-key".to_string()),
        concurrency: 1,
        ping_interval: Duration::from_secs(10),
        request_timeout: Duration::from_secs(5),
    };
    let worker = ServerlessWorker::new(config).unwrap();

    Mock::given(method("GET"))
        .and(path("/job-take/worker-1"))
        .and(query_param("token", "abc"))
        .and(query_param("job_in_progress", "0"))
        .and(header("authorization", "worker-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "job-1",
            "input": {"text": "hello"}
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/job-done/worker-1/job-1"))
        .and(query_param("token", "abc"))
        .and(query_param("isStream", "false"))
        .and(header("authorization", "worker-key"))
        .and(header("x-request-id", "job-1"))
        .and(body_string_contains("\"output\":{\"ok\":true}"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let processed = worker
        .run_once(&|_job| async move { Ok(json!({"ok": true})) })
        .await
        .unwrap();

    assert!(processed);
}

#[tokio::test]
async fn worker_heartbeat_sends_active_jobs_as_comma_separated_query_value() {
    let server = MockServer::start().await;
    let config = WorkerConfig {
        worker_id: "worker-1".to_string(),
        get_job_url: format!("{}/job-take/worker-1?token=abc", server.uri()),
        post_output_url: format!("{}/job-done/$ID?token=abc", server.uri()),
        ping_url: Some(format!("{}/ping/worker-1?token=abc", server.uri())),
        api_key: Some("worker-key".to_string()),
        concurrency: 1,
        ping_interval: Duration::from_millis(20),
        request_timeout: Duration::from_secs(5),
    };
    let worker = ServerlessWorker::new(config).unwrap();

    Mock::given(method("GET"))
        .and(path("/job-take/worker-1"))
        .and(query_param("token", "abc"))
        .and(query_param("job_in_progress", "0"))
        .and(header("authorization", "worker-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "job-1",
            "input": {"text": "hello"}
        })))
        .expect(1..)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/ping/worker-1"))
        .and(query_param("token", "abc"))
        .and(query_param("job_id", "job-1"))
        .and(header("authorization", "worker-key"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1..)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/job-done/job-1"))
        .and(query_param("token", "abc"))
        .and(query_param("isStream", "false"))
        .and(header("authorization", "worker-key"))
        .and(header("x-request-id", "job-1"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let call_count = Arc::new(AtomicUsize::new(0));
    let handler_call_count = Arc::clone(&call_count);
    let worker_task = tokio::spawn(async move {
        worker
            .run(move |_job| {
                let handler_call_count = Arc::clone(&handler_call_count);
                async move {
                    if handler_call_count.fetch_add(1, Ordering::SeqCst) == 0 {
                        tokio::time::sleep(Duration::from_millis(75)).await;
                        Ok(json!({"ok": true}))
                    } else {
                        future::pending::<Result<serde_json::Value, String>>().await
                    }
                }
            })
            .await
    });

    tokio::time::sleep(Duration::from_millis(100)).await;
    worker_task.abort();
}

#[tokio::test]
async fn worker_run_processes_jobs_concurrently_up_to_configured_concurrency() {
    let server = MockServer::start().await;
    let config = WorkerConfig {
        worker_id: "worker-1".to_string(),
        get_job_url: format!("{}/job-take/worker-1", server.uri()),
        post_output_url: format!("{}/job-done/$ID", server.uri()),
        ping_url: None,
        api_key: None,
        concurrency: 2,
        ping_interval: Duration::from_secs(10),
        request_timeout: Duration::from_secs(5),
    };
    let worker = ServerlessWorker::new(config).unwrap();

    // Every poll gets a job — with `concurrency: 2`, `run` should keep two
    // of `run`'s internal poll/process loops going side by side, so two
    // handler invocations should be in flight at once at some point.
    Mock::given(method("GET"))
        .and(path("/job-take/worker-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "job-1",
            "input": {}
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/job-done/job-1"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let concurrent = Arc::new(AtomicUsize::new(0));
    let max_concurrent = Arc::new(AtomicUsize::new(0));
    let handler_concurrent = Arc::clone(&concurrent);
    let handler_max_concurrent = Arc::clone(&max_concurrent);

    let worker_task = tokio::spawn(async move {
        worker
            .run(move |_job| {
                let concurrent = Arc::clone(&handler_concurrent);
                let max_concurrent = Arc::clone(&handler_max_concurrent);
                async move {
                    let now = concurrent.fetch_add(1, Ordering::SeqCst) + 1;
                    max_concurrent.fetch_max(now, Ordering::SeqCst);
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    concurrent.fetch_sub(1, Ordering::SeqCst);
                    Ok(json!({"ok": true}))
                }
            })
            .await
    });

    tokio::time::sleep(Duration::from_millis(150)).await;
    worker_task.abort();

    assert_eq!(
        max_concurrent.load(Ordering::SeqCst),
        2,
        "expected 2 handler invocations to overlap with concurrency=2"
    );
}

#[tokio::test]
async fn worker_run_prefetches_next_job_while_handler_is_running() {
    let server = MockServer::start().await;
    let config = WorkerConfig {
        worker_id: "worker-1".to_string(),
        get_job_url: format!("{}/job-take/worker-1", server.uri()),
        post_output_url: format!("{}/job-done/$ID", server.uri()),
        ping_url: None,
        api_key: None,
        concurrency: 1,
        ping_interval: Duration::from_secs(10),
        request_timeout: Duration::from_secs(5),
    };
    let worker = ServerlessWorker::new(config).unwrap();

    // The initial "I'm free" poll (`job_in_progress=0`) hands out job-1.
    // Bounded below, not exactly — once job-2 (and whatever the loop polls
    // for afterwards) is in flight, further iterations may or may not land
    // another `job_in_progress=0` poll within this test's timing window;
    // only the first two entries in `order` (checked below) matter.
    Mock::given(method("GET"))
        .and(path("/job-take/worker-1"))
        .and(query_param("job_in_progress", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "job-1",
            "input": {}
        })))
        .expect(1..)
        .mount(&server)
        .await;

    // The prefetch poll (`job_in_progress=1`) — issued while job-1's
    // handler is still running — hands out job-2. This is the behavior
    // under test: if `run`'s loop only polled after `post_result`
    // completed (no prefetch), this mock would never be hit before job-1
    // finishes, and job-2 wouldn't be ready the instant job-1 is done.
    Mock::given(method("GET"))
        .and(path("/job-take/worker-1"))
        .and(query_param("job_in_progress", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "job-2",
            "input": {}
        })))
        .expect(1..)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/job-done/job-1"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1..)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/job-done/job-2"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let order = Arc::new(std::sync::Mutex::new(Vec::new()));
    let handler_order = Arc::clone(&order);

    let worker_task = tokio::spawn(async move {
        worker
            .run(move |job| {
                let order = Arc::clone(&handler_order);
                async move {
                    order.lock().unwrap().push(job.id.clone());
                    if job.id == "job-1" {
                        // Long enough that the concurrent prefetch poll for
                        // job-2 has time to land before this returns.
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                    Ok(json!({"ok": true}))
                }
            })
            .await
    });

    tokio::time::sleep(Duration::from_millis(100)).await;
    worker_task.abort();

    let order = order.lock().unwrap();
    assert!(
        order.len() >= 2,
        "expected at least job-1 and job-2 to start, got {order:?}"
    );
    assert_eq!(
        order[..2],
        ["job-1".to_string(), "job-2".to_string()],
        "job-2 should start right after job-1 finishes, using the prefetched job \
         (not a fresh job_in_progress=0 poll after job-1's post_result)"
    );
}

#[tokio::test]
async fn worker_run_reports_job_in_progress_from_every_loop_while_a_job_is_active() {
    // With `concurrency > 1`, the loops that are not running the job keep
    // polling. If they poll with `job_in_progress=0`, RunPod reads the worker
    // as idle, abandons the running job after ~60 s and retries it ("job
    // timed out after 1 retries"), so every job longer than that fails.
    // While any job is active, every poll must say `job_in_progress=1`.
    let server = MockServer::start().await;
    let config = WorkerConfig {
        worker_id: "worker-1".to_string(),
        get_job_url: format!("{}/job-take/worker-1", server.uri()),
        post_output_url: format!("{}/job-done/$ID", server.uri()),
        ping_url: None,
        api_key: None,
        concurrency: 2,
        ping_interval: Duration::from_secs(10),
        request_timeout: Duration::from_secs(5),
    };
    let worker = ServerlessWorker::new(config).unwrap();

    // The first "I'm free" poll hands out job-1; every later poll gets no job.
    Mock::given(method("GET"))
        .and(path("/job-take/worker-1"))
        .and(query_param("job_in_progress", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "job-1",
            "input": {}
        })))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/job-take/worker-1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let started = Arc::new(AtomicUsize::new(0));
    let handler_started = Arc::clone(&started);
    let worker_task = tokio::spawn(async move {
        worker
            .run(move |_job| {
                let started = Arc::clone(&handler_started);
                async move {
                    started.fetch_add(1, Ordering::SeqCst);
                    // Still running when the worker is aborted below.
                    tokio::time::sleep(Duration::from_secs(10)).await;
                    Ok(json!({"ok": true}))
                }
            })
            .await
    });

    // Long enough for the idle loop's 1 s retry sleep to elapse at least once.
    tokio::time::sleep(Duration::from_millis(2500)).await;
    worker_task.abort();

    assert_eq!(started.load(Ordering::SeqCst), 1, "job-1 should be running");
    let polls: Vec<String> = server
        .received_requests()
        .await
        .unwrap()
        .into_iter()
        .filter(|request| request.url.path() == "/job-take/worker-1")
        .map(|request| {
            request
                .url
                .query_pairs()
                .find(|(key, _)| key == "job_in_progress")
                .map(|(_, value)| value.into_owned())
                .unwrap_or_default()
        })
        .collect();
    // Each loop's first poll can race ahead of job-1 being handed out, so
    // one idle poll per loop is legitimate; every poll after those happens
    // while job-1 is active and must report 1.
    let concurrency = 2;
    assert!(
        polls.len() > concurrency,
        "expected the second loop to keep polling while job-1 runs, got {polls:?}"
    );
    assert!(
        polls[concurrency..].iter().all(|value| value == "1"),
        "polls while job-1 is active must report job_in_progress=1, got {polls:?}"
    );
}

#[tokio::test]
async fn worker_returns_false_when_no_job_is_available() {
    let server = MockServer::start().await;
    let config = WorkerConfig {
        worker_id: "worker-1".to_string(),
        get_job_url: format!("{}/job-take/worker-1", server.uri()),
        post_output_url: format!("{}/job-done/$ID", server.uri()),
        ping_url: None,
        api_key: None,
        concurrency: 1,
        ping_interval: Duration::from_secs(10),
        request_timeout: Duration::from_secs(5),
    };
    let worker = ServerlessWorker::new(config).unwrap();

    Mock::given(method("GET"))
        .and(path("/job-take/worker-1"))
        .and(query_param("job_in_progress", "0"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let processed = worker
        .run_once(&|_job| async move { Ok(json!({"unused": true})) })
        .await
        .unwrap();

    assert!(!processed);
}
