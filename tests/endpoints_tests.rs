use runpod_sdk::{RunpodClient, RunpodConfig};
use runpod_sdk::service::EndpointsService;
use runpod_sdk::model::{EndpointCreateInput, EndpointUpdateInput};
use wiremock::matchers::{method, path, header};
use wiremock::{Mock, MockServer, ResponseTemplate};
use serde_json::json;

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
        "gpuIds": "AMPERE_16",
        "networkVolumeId": null,
        "workersMin": 0,
        "workersMax": 1,
        "idleTimeout": 5,
        "flashboot": false,
        "templateId": "tmpl-123"
    });

    Mock::given(method("GET"))
        .and(path("/endpoints/ep-123"))
        .and(header("authorization", "Bearer test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let endpoint = client.get_endpoint("ep-123", Default::default()).await.expect("Failed to get endpoint");
    
    assert_eq!(endpoint.id, "ep-123");
    assert_eq!(endpoint.name, "Test Endpoint");
    assert_eq!(endpoint.workers_min, 0);
}
