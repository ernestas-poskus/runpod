use runpod_sdk::RunpodConfig;
use runpod_sdk::service::EndpointsService;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_endpoint() {
    let mock_server = MockServer::start().await;

    let client = RunpodConfig::builder()
        .with_api_key("test-api-key")
        .with_rest_url(&mock_server.uri())
        .build_client()
        .expect("Failed to build client");

    let response_body = json!({
        "id": "ep-123",
        "name": "Test Endpoint",
        "userId": "user-123",
        "templateId": "tmpl-123",
        "version": 1,
        "computeType": "GPU",
        "createdAt": "2026-07-07T16:00:00Z",
        "dataCenterIds": ["EU-RO-1"],
        "executionTimeoutMs": 30000,
        "idleTimeout": 5,
        "scalerType": "QUEUE_DELAY",
        "scalerValue": 10,
        "workersMax": 1,
        "workersMin": 0
    });

    Mock::given(method("GET"))
        .and(path("/endpoints/ep-123"))
        .and(header("authorization", "Bearer test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let endpoint = client
        .get_endpoint("ep-123", Default::default())
        .await
        .expect("Failed to get endpoint");

    assert_eq!(endpoint.id, "ep-123");
    assert_eq!(endpoint.name.as_deref(), Some("Test Endpoint"));
    assert_eq!(endpoint.workers_min, 0);
}
