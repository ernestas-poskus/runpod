# Runpod SDK

[![Crates.io](https://img.shields.io/crates/v/runpod-sdk?style=flat-square&color=black)](https://crates.io/crates/runpod-sdk)
[![Documentation](https://img.shields.io/docsrs/runpod-sdk?style=flat-square&color=black)](https://docs.rs/runpod-sdk)
[![Build](https://img.shields.io/github/actions/workflow/status/ernestas-poskus/runpod/build.yml?style=flat-square&color=black)](https://github.com/ernestas-poskus/runpod/actions)

A Rust client library for the [Runpod API](https://docs.runpod.io/). This SDK
provides a type-safe, ergonomic interface for managing Pods, Serverless
endpoints, templates, network volumes, and more.

This crate is a fork of the original
[runpod.rs](https://github.com/agentsea/runpod.rs) developed by
[Patrick Barker](https://github.com/pbarker).

## Features

- **Serverless Endpoints**: Full endpoint management with configuration
- **Type Safety**: Strongly typed models with comprehensive validation
- **Async/Await**: Built on modern async Rust with `tokio` and `reqwest`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
runpod-sdk = { version = "0.2", features = ["serverless"] }
```

## Quick Start

### Builder Configuration

```rust,no_run
use runpod_sdk::{RunpodClient, Result};
use runpod_sdk::service::PodsService;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let client = RunpodClient::builder()
        .with_api_key("your-api-key")
        .with_timeout(Duration::from_secs(60))
        .build_client()?;

    let pods = client.list_pods(Default::default()).await?;
    println!("Found {} pods", pods.len());

    Ok(())
}
```

### Environment Variables

The SDK can be configured using environment variables:

| Variable              | Required | Default                     | Description                                                                              |
| --------------------- | -------- | --------------------------- | ---------------------------------------------------------------------------------------- |
| `RUNPOD_API_KEY`      | Yes      | -                           | Your RunPod API key from [console settings](https://www.runpod.io/console/user/settings) |
| `RUNPOD_REST_URL`     | No       | `https://rest.runpod.io/v1` | Custom REST API base URL                                                                 |
| `RUNPOD_API_URL`      | No       | `https://api.runpod.io/v2`  | Custom API URL for serverless endpoints (requires `serverless` feature)                  |
| `RUNPOD_TIMEOUT_SECS` | No       | `30`                        | Request timeout in seconds (max: 300)                                                    |

```rust,no_run
use runpod_sdk::{RunpodClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client: RunpodClient = RunpodClient::from_env()?;
    Ok(())
}
```

## Optional Features

### TLS Backend

Choose between two TLS implementations:

```toml
# Default: rustls-tls (recommended)
runpod-sdk = { version = "0.2", features = [] }

# Alternative: native-tls
runpod-sdk = { version = "0.2", features = ["native-tls"], default-features = false }
```

### Tracing Support

Enable comprehensive logging and tracing via the [`tracing`](https://crates.io/crates/tracing) crate. 
Tracing targets are defined in `lib.rs` for fine-grained control over log output:

```toml
runpod-sdk = { version = "0.2", features = ["tracing"] }
```

## Examples

The `examples/` directory contains comprehensive usage examples:

```bash
# Set your API key
export RUNPOD_API_KEY="your-api-key"

# Run the basic usage example
cargo run --example basic_usage

# Run the endpoint lifecycle management example
cargo run --example manage_endpoint

# Run the serverless endpoint example (requires serverless feature)
export RUNPOD_ENDPOINT_ID="your-endpoint-id"
cargo run --example run_endpoint --features serverless
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md)
for details on how to submit pull requests, report issues, and contribute to the
project.

## License

This project is licensed under the MIT License - see the
[LICENSE.txt](LICENSE.txt) file for details.

## Resources

- [Runpod Documentation](https://docs.runpod.io/)
- [Runpod API Reference](https://rest.runpod.io/v1/docs)
- [Full API Documentation](https://docs.rs/runpod-sdk)
