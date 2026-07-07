#![cfg(feature = "serverless")]

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
