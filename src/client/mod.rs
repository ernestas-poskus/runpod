//! RunPod API client configuration and initialization.
//!
//! This module provides the core client types for interacting with the RunPod API:
//!
//! - [`RunpodConfig`] - Configuration builder for API settings
//! - [`RunpodBuilder`] - Builder pattern for creating configurations
//! - [`RunpodClient`] - Main client for making API requests

mod config;
mod runpod;

pub use config::RunpodConfig;
pub use runpod::RunpodClient;

/// Configuration builder types for RunPod clients.
///
/// This module contains the builder pattern types used to construct
/// [`RunpodConfig`](super::RunpodConfig) instances with custom settings.
///
/// # Examples
///
/// ```no_run
/// use runpod_sdk::builder::RunpodBuilder;
///
/// let config = RunpodBuilder::default()
///     .with_api_key("your-api-key")
///     .build()
///     .unwrap();
/// ```
pub mod builder {
    pub use super::config::{RunpodBuilder, RunpodBuilderError};
}
