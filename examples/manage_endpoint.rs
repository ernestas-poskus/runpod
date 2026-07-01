//! Endpoint lifecycle management example.
//!
//! Demonstrates creating, updating, and deleting a serverless endpoint.
//!
//! # Usage
//!
//! ```bash
//! export RUNPOD_API_KEY="your-api-key-here"
//! cargo run --example manage_endpoint
//! ```

use runpod_sdk::model::{EndpointCreateInput, EndpointUpdateInput};
use runpod_sdk::service::EndpointsService;
use runpod_sdk::{Result, RunpodClient};

#[tokio::main]
async fn main() -> Result<()> {
    let client = RunpodClient::from_env()?;

    // Create a new endpoint
    println!("Creating endpoint...");
    let create_input = EndpointCreateInput {
        template_id: "your-template-id".to_string(),
        name: Some("test-endpoint".to_string()),
        workers_min: Some(0),
        workers_max: Some(3),
        ..Default::default()
    };

    let endpoint = client.create_endpoint(create_input).await?;
    println!(
        "Created endpoint: {} ({})",
        endpoint.name.as_deref().unwrap_or("unnamed"),
        endpoint.id
    );

    // Update the endpoint
    println!("\nUpdating endpoint...");
    let update_input = EndpointUpdateInput {
        name: Some("updated-test-endpoint".to_string()),
        workers_max: Some(5),
        ..Default::default()
    };

    client.update_endpoint(&endpoint.id, update_input).await?;
    println!("Endpoint updated");

    // Delete the endpoint
    println!("\nDeleting endpoint...");
    client.delete_endpoint(&endpoint.id).await?;
    println!("Endpoint deleted");

    Ok(())
}
