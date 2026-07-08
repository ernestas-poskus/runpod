use runpod_sdk::RunpodConfig;
use runpod_sdk::model::{NetworkVolumeCreateInput, NetworkVolumeUpdateInput};
use runpod_sdk::service::VolumesService;
use serde_json::json;
use wiremock::matchers::{body_partial_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_create_volume() {
    let mock_server = MockServer::start().await;

    let client = RunpodConfig::builder()
        .with_api_key("test-api-key")
        .with_rest_url(mock_server.uri())
        .build_client()
        .expect("Failed to build client");

    let input = NetworkVolumeCreateInput {
        name: "test-volume".to_string(),
        size: 50,
        data_center_id: "EU-RO-1".to_string(),
    };

    let response_body = json!({
        "id": "vol-123",
        "name": "test-volume",
        "size": 50,
        "dataCenterId": "EU-RO-1"
    });

    Mock::given(method("POST"))
        .and(path("/networkvolumes"))
        .and(header("authorization", "Bearer test-api-key"))
        .and(body_partial_json(json!({
            "name": "test-volume",
            "size": 50,
            "dataCenterId": "EU-RO-1"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let volume = client
        .create_volume(input)
        .await
        .expect("Failed to create volume");

    assert_eq!(volume.id, "vol-123");
    assert_eq!(volume.name, "test-volume");
    assert_eq!(volume.size, 50);
    assert_eq!(volume.data_center_id, "EU-RO-1");
}

#[tokio::test]
async fn test_list_volumes() {
    let mock_server = MockServer::start().await;

    let client = RunpodConfig::builder()
        .with_api_key("test-api-key")
        .with_rest_url(mock_server.uri())
        .build_client()
        .expect("Failed to build client");

    let response_body = json!([
        {
            "id": "vol-123",
            "name": "test-volume-1",
            "size": 50,
            "dataCenterId": "EU-RO-1"
        },
        {
            "id": "vol-456",
            "name": "test-volume-2",
            "size": 100,
            "dataCenterId": "US-TX-1"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/networkvolumes"))
        .and(header("authorization", "Bearer test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let volumes = client.list_volumes().await.expect("Failed to list volumes");

    assert_eq!(volumes.len(), 2);
    assert_eq!(volumes[0].id, "vol-123");
    assert_eq!(volumes[1].id, "vol-456");
}

#[tokio::test]
async fn test_get_volume() {
    let mock_server = MockServer::start().await;

    let client = RunpodConfig::builder()
        .with_api_key("test-api-key")
        .with_rest_url(mock_server.uri())
        .build_client()
        .expect("Failed to build client");

    let response_body = json!({
        "id": "vol-123",
        "name": "test-volume",
        "size": 50,
        "dataCenterId": "EU-RO-1"
    });

    Mock::given(method("GET"))
        .and(path("/networkvolumes/vol-123"))
        .and(header("authorization", "Bearer test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let volume = client
        .get_volume("vol-123")
        .await
        .expect("Failed to get volume");

    assert_eq!(volume.id, "vol-123");
    assert_eq!(volume.name, "test-volume");
}

#[tokio::test]
async fn test_update_volume() {
    let mock_server = MockServer::start().await;

    let client = RunpodConfig::builder()
        .with_api_key("test-api-key")
        .with_rest_url(mock_server.uri())
        .build_client()
        .expect("Failed to build client");

    let input = NetworkVolumeUpdateInput {
        name: Some("updated-volume-name".to_string()),
        size: Some(100),
    };

    let response_body = json!({
        "id": "vol-123",
        "name": "updated-volume-name",
        "size": 100,
        "dataCenterId": "EU-RO-1"
    });

    Mock::given(method("PATCH"))
        .and(path("/networkvolumes/vol-123"))
        .and(header("authorization", "Bearer test-api-key"))
        .and(body_partial_json(json!({
            "name": "updated-volume-name",
            "size": 100
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let volume = client
        .update_volume("vol-123", input)
        .await
        .expect("Failed to update volume");

    assert_eq!(volume.id, "vol-123");
    assert_eq!(volume.name, "updated-volume-name");
    assert_eq!(volume.size, 100);
}

#[tokio::test]
async fn test_delete_volume() {
    let mock_server = MockServer::start().await;

    let client = RunpodConfig::builder()
        .with_api_key("test-api-key")
        .with_rest_url(mock_server.uri())
        .build_client()
        .expect("Failed to build client");

    Mock::given(method("DELETE"))
        .and(path("/networkvolumes/vol-123"))
        .and(header("authorization", "Bearer test-api-key"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&mock_server)
        .await;

    client
        .delete_volume("vol-123")
        .await
        .expect("Failed to delete volume");
}
