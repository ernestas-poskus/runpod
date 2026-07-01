//! Basic usage example demonstrating simple queries.
//!
//! # Usage
//!
//! ```bash
//! export RUNPOD_API_KEY="your-api-key-here"
//! cargo run --example basic_usage
//! ```

use runpod_sdk::model::{ListEndpointsQuery, ListPodsQuery, ListTemplatesQuery};
use runpod_sdk::service::{EndpointsService, PodsService, TemplatesService};
use runpod_sdk::{Result, RunpodClient};

#[tokio::main]
async fn main() -> Result<()> {
    let client = RunpodClient::from_env()?;

    // List all pods
    let pods = client.list_pods(ListPodsQuery::default()).await?;
    println!("Found {} pods", pods.len());

    // List endpoints
    let endpoints = client.list_endpoints(ListEndpointsQuery::default()).await?;
    println!("Found {} endpoints", endpoints.len());

    // List templates
    let templates = client.list_templates(ListTemplatesQuery::default()).await?;
    println!("Found {} templates", templates.len());

    Ok(())
}
