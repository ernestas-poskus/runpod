//! Service traits for RunPod API resource management.
//!
//! This module provides service traits that extend [`RunpodClient`](crate::RunpodClient)
//! with methods for managing specific resource types: serverless endpoints, pods,
//! templates, volumes, container registries, and billing information.

mod billing;
mod endpoints;
mod pods;
mod registry;
mod templates;
mod volumes;

pub use billing::BillingService;
pub use endpoints::EndpointsService;
pub use pods::PodsService;
pub use registry::RegistryService;
pub use templates::TemplatesService;
pub use volumes::VolumesService;
