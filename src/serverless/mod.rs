//! Serverless endpoint execution and job management.
//!
//! This module provides functionality for running jobs on RunPod serverless endpoints,
//! including synchronous runs, streaming outputs, and asynchronous job status tracking.

mod client;
mod job;
mod types;

pub use client::ServerlessEndpoint;
pub use job::ServerlessJob;
pub use types::{EndpointHealth, JobOutput, JobStatus, StreamChunk};
pub(crate) use types::{JobStatusResponse, RunRequest, RunResponse, StreamResponse};
