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
