//! Prelude module for convenient imports.
//!
//! The prelude re-exports the most commonly used types and traits from the RunPod SDK,
//! allowing you to import everything you need with a single glob import.

pub use crate::builder::{RunpodBuilder, RunpodBuilderError};
pub use crate::model::*;
#[cfg(feature = "serverless")]
pub use crate::serverless::{
    EndpointHealth, JobOutput, JobStatus, ServerlessEndpoint, ServerlessJob, StreamChunk,
};
pub use crate::service::{
    BillingService, EndpointsService, PodsService, RegistryService, TemplatesService,
    VolumesService,
};
pub use crate::{Error, Result, RunpodClient, RunpodConfig};
