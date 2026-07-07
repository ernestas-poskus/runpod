use runpod_sdk::RunpodConfig;
use runpod_sdk::model::{
    BillingGrouping, BucketSize, EndpointBillingQuery, NetworkVolumeBillingQuery, PodBillingQuery,
};
use runpod_sdk::service::BillingService;
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_pod_billing() {
    let mock_server = MockServer::start().await;

    let client = RunpodConfig::builder()
        .with_api_key("test-api-key")
        .with_rest_url(&mock_server.uri())
        .build_client()
        .expect("Failed to build client");

    let response_body = json!([
        {
            "amount": 12.34,
            "podId": "pod-123",
            "time": "2026-07-07T16:00:00Z",
            "timeBilledMs": 3600000
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/billing/pods"))
        .and(query_param("bucketSize", "day"))
        .and(query_param("grouping", "podId"))
        .and(header("authorization", "Bearer test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let query = PodBillingQuery {
        bucket_size: Some(BucketSize::Day),
        grouping: Some(BillingGrouping::PodId),
        ..Default::default()
    };

    let records = client
        .get_pod_billing(query)
        .await
        .expect("Failed to get pod billing");

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].amount, 12.34);
    assert_eq!(records[0].pod_id.as_deref(), Some("pod-123"));
    assert_eq!(records[0].time, "2026-07-07T16:00:00Z");
    assert_eq!(records[0].time_billed_ms, Some(3600000));
}

#[tokio::test]
async fn test_get_endpoint_billing() {
    let mock_server = MockServer::start().await;

    let client = RunpodConfig::builder()
        .with_api_key("test-api-key")
        .with_rest_url(&mock_server.uri())
        .build_client()
        .expect("Failed to build client");

    let response_body = json!([
        {
            "amount": 5.67,
            "endpointId": "ep-123",
            "time": "2026-07-07T16:00:00Z",
            "timeBilledMs": 1800000
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/billing/endpoints"))
        .and(query_param("bucketSize", "hour"))
        .and(query_param("grouping", "endpointId"))
        .and(header("authorization", "Bearer test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let query = EndpointBillingQuery {
        bucket_size: Some(BucketSize::Hour),
        grouping: Some(BillingGrouping::EndpointId),
        ..Default::default()
    };

    let records = client
        .get_endpoint_billing(query)
        .await
        .expect("Failed to get endpoint billing");

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].amount, 5.67);
    assert_eq!(records[0].endpoint_id.as_deref(), Some("ep-123"));
    assert_eq!(records[0].time, "2026-07-07T16:00:00Z");
    assert_eq!(records[0].time_billed_ms, Some(1800000));
}

#[tokio::test]
async fn test_get_volume_billing() {
    let mock_server = MockServer::start().await;

    let client = RunpodConfig::builder()
        .with_api_key("test-api-key")
        .with_rest_url(&mock_server.uri())
        .build_client()
        .expect("Failed to build client");

    let response_body = json!([
        {
            "amount": 1.23,
            "time": "2026-07-07T16:00:00Z",
            "diskSpaceBilledGb": 100
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/billing/networkvolumes"))
        .and(header("authorization", "Bearer test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let query = NetworkVolumeBillingQuery::default();

    let records = client
        .get_volume_billing(query)
        .await
        .expect("Failed to get volume billing");

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].amount, 1.23);
    assert_eq!(records[0].disk_space_billed_gb, Some(100));
    assert_eq!(records[0].time, "2026-07-07T16:00:00Z");
}
