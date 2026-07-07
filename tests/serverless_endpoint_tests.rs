#![cfg(feature = "serverless")]

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use runpod_sdk::RunpodConfig;
use runpod_sdk::serverless::ServerlessEndpoint;
use serde_json::json;
use wiremock::matchers::{body_partial_json, header, method, path};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

struct PollingResponder {
    counter: Arc<AtomicUsize>,
}

impl Respond for PollingResponder {
    fn respond(&self, _request: &Request) -> ResponseTemplate {
        let count = self.counter.fetch_add(1, Ordering::SeqCst);
        if count == 0 {
            ResponseTemplate::new(200).set_body_json(json!({
                "id": "job-123",
                "status": "IN_PROGRESS"
            }))
        } else {
            ResponseTemplate::new(200).set_body_json(json!({
                "id": "job-123",
                "status": "COMPLETED",
                "output": {"result": "success-data"}
            }))
        }
    }
}

#[tokio::test]
async fn test_serverless_endpoint_run_and_await() {
    let mock_server = MockServer::start().await;

    let client = RunpodConfig::builder()
        .with_api_key("test-api-key")
        .with_api_url(&mock_server.uri())
        .build_client()
        .expect("Failed to build client");

    let endpoint = ServerlessEndpoint::new("ep-123", client);

    // Mock post to /run
    Mock::given(method("POST"))
        .and(path("/ep-123/run"))
        .and(header("authorization", "Bearer test-api-key"))
        .and(body_partial_json(json!({
            "input": {"prompt": "Hello"}
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "job-123",
            "status": "IN_QUEUE"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Mock status poll with our sequential responder
    let counter = Arc::new(AtomicUsize::new(0));
    Mock::given(method("GET"))
        .and(path("/ep-123/status/job-123"))
        .and(header("authorization", "Bearer test-api-key"))
        .respond_with(PollingResponder { counter })
        .expect(2)
        .mount(&mock_server)
        .await;

    // Submit the job
    let job = endpoint
        .run(&json!({"prompt": "Hello"}))
        .expect("Failed to create job");

    // Await the job completion
    let output: serde_json::Value = job.await.expect("Job execution failed");

    assert_eq!(output, json!({"result": "success-data"}));
}
