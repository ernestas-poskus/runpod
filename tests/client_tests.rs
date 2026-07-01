use runpod_sdk::{RunpodClient, RunpodConfig};
use runpod_sdk::service::PodsService;
use runpod_sdk::model::PodCreateInput;
use wiremock::matchers::{method, path, header};
use wiremock::{Mock, MockServer, ResponseTemplate};
use serde_json::json;

#[tokio::test]
async fn test_list_pods() {
    let mock_server = MockServer::start().await;
    
    let client = RunpodConfig::builder()
        .with_api_key("test-api-key")
        .with_rest_url(&mock_server.uri())
        .build_client()
        .expect("Failed to build client");

    let response_body = json!([
        {
            "id": "pod-123",
            "image": "runpod/pytorch:latest",
            "consumerUserId": "user-123",
            "machineId": "machine-123",
            "desiredStatus": "RUNNING",
            "costPerHr": 0.5,
            "adjustedCostPerHr": 0.5,
            "vcpuCount": 4.0,
            "memoryInGb": 16.0,
            "containerDiskInGb": 20,
            "volumeEncrypted": false,
            "ports": [],
            "env": {},
            "interruptible": false,
            "locked": false
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/pods"))
        .and(header("authorization", "Bearer test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let pods = client.list_pods(Default::default()).await.expect("Failed to list pods");
    
    assert_eq!(pods.len(), 1);
    assert_eq!(pods[0].id, "pod-123");
    assert_eq!(pods[0].image, "runpod/pytorch:latest");
}

#[tokio::test]
async fn test_create_pod() {
    let mock_server = MockServer::start().await;
    
    let client = RunpodConfig::builder()
        .with_api_key("test-api-key")
        .with_rest_url(&mock_server.uri())
        .build_client()
        .expect("Failed to build client");

    let response_body = json!({
        "id": "pod-456",
        "image": "runpod/ubuntu:latest",
        "consumerUserId": "user-123",
        "machineId": "machine-123",
        "desiredStatus": "RUNNING",
        "costPerHr": 0.2,
        "adjustedCostPerHr": 0.2,
        "vcpuCount": 2.0,
        "memoryInGb": 8.0,
        "containerDiskInGb": 10,
        "volumeEncrypted": false,
        "ports": [],
        "env": {},
        "interruptible": false,
        "locked": false
    });

    Mock::given(method("POST"))
        .and(path("/pods"))
        .and(header("authorization", "Bearer test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let input = PodCreateInput {
        image_name: Some("runpod/ubuntu:latest".to_string()),
        ..Default::default()
    };
    
    let pod = client.create_pod(input).await.expect("Failed to create pod");
    
    assert_eq!(pod.id, "pod-456");
    assert_eq!(pod.image, "runpod/ubuntu:latest");
}
