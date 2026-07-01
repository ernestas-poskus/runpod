//! Example demonstrating how to run jobs on RunPod serverless endpoints.
//!
//! # Usage
//!
//! ```bash
//! export RUNPOD_API_KEY="your-api-key-here"
//! export RUNPOD_ENDPOINT_ID="your-endpoint-id"
//! cargo run --example run_endpoint --features serverless
//! ```

use runpod_sdk::serverless::ServerlessEndpoint;
use runpod_sdk::{Result, RunpodClient};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint_id = std::env::var("RUNPOD_ENDPOINT_ID")
        .expect("RUNPOD_ENDPOINT_ID environment variable not set");

    let client = RunpodClient::from_env()?;
    let endpoint = ServerlessEndpoint::new(endpoint_id, client);

    // Check endpoint health
    let health = endpoint.health().await?;
    println!(
        "Jobs: {} completed, {} in queue",
        health.jobs.completed, health.jobs.in_queue
    );
    println!(
        "Workers: {} ready, {} running",
        health.workers.ready, health.workers.running
    );

    // Run a job
    let input = json!({"prompt": "Hello, World!"});
    let job = endpoint.run(&input)?;
    let output = job.await?;
    println!("Job output: {:?}", output);

    // Stream job results
    let input = json!({"prompt": "Generate streaming output"});
    let job = endpoint.run(&input)?;

    loop {
        let (status, chunks) = job.stream().await?;

        if !chunks.is_empty() {
            println!("Received {} chunk(s)", chunks.len());
        }

        if status.is_final() {
            println!("Stream completed: {:?}", status);
            break;
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // Cancel a job
    let input = json!({"prompt": "This job will be cancelled"});
    let job = endpoint.run(&input)?;
    job.cancel().await?;
    println!("Job cancelled");

    Ok(())
}
