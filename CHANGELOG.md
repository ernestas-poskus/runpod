# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- N/A

### Changed

- N/A

### Deprecated

- N/A

### Removed

- N/A

### Fixed

- N/A

### Security

- N/A

## [0.2.2] - 2025-12-08

### Fixed

- Fixed builder module tests

### Removed

- Removed redundant `RunpodBuilder` and `RunpodBuilderError` exports from crate
  root (still accessible via `builder` module)

## [0.2.1] - 2025-12-07

### Added

- Added `client::builder` module containing `RunpodBuilder` and
  `RunpodBuilderError`

### Changed

- Improved crate description for better discoverability
- Updated module documentation for better clarity

## [0.2.0] - 2025-12-07

### Added

- Added serverless module exports to prelude for convenient imports

### Removed

- **BREAKING**: Removed GraphQL support and `graphql` feature flag
  - Removed `RunpodClient::graphql_query()` method
  - Removed `RUNPOD_GRAPHQL_URL` environment variable support
  - Removed GraphQL-related configuration from `RunpodConfig`

## [0.1.0] - 2025-11-06

### Added

- Initial implementation of Runpod Rust SDK
- Full support for Runpod REST API endpoints
- `RunpodConfig` and `RunpodBuilder` for client configuration
- Service modules for managing resources
- Comprehensive type-safe models for all API resources
- Builder pattern support for configuration via `derive_builder`
- Optional `tracing` feature for logging support

[Unreleased]: https://github.com/ernestas-poskus/runpod/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/ernestas-poskus/runpod/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/ernestas-poskus/runpod/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/ernestas-poskus/runpod/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ernestas-poskus/runpod/releases/tag/v0.1.0
