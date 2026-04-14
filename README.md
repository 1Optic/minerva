# Minerva ETL Platform

High-performance ETL platform for standardized time series, attribute, and event type data.

Minerva has been in use by Tier 1 telecom Communication Service Providers (CSP) for over more than 15 years. It offers real-time insight in your network data, no matter the scale or volume. Interested in the possibilities? Feel free to [book a demo at 1Optic.io](https://1optic.io)!

## Features

The standardization of time series, attribute, and event type data types makes operations like aggregations and KPI calculations trivial and possible on both off- and online granularities. The combination of these standardized types of data enables advanced correlation capabilities.

Using stable modern technologies like PostgreSQL, Citus and Rust, Minerva provides you with:

- Low latency
- Robustness
- Scalability
- Vendor independence
- Openness
- Security

## Prerequisites

Before installing or developing with Minerva ETL, make sure the following dependencies are available on your system:

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo
- `clang`, required for building the `pg_query` crate

Depending on your local setup, you may also need:

- Docker, if your test setup uses containers
- A local PostgreSQL environment, if you are developing against a database directly

## Installation

To install the Minerva CLI locally using Cargo, run:

```sh
cargo install --path crates/cli
```

After installation, verify that the CLI is available:

```sh
$ minerva --version
minerva 9.44.1
```

## Quick Start

To start a Minerva database initialized with a test instance (requires Docker), run:

```sh
minerva start --with-definition examples/tiny_instance_v1
```

This command starts a local Minerva instance using the example definition located in `examples/tiny_instance_v1`.

This is useful for:

- Local development
- Testing changes
- Exploring the platform setup
- Verifying that your environment is configured correctly

## Running Tests

Unit tests and integration tests can be run with:

```sh
cargo test
```

Test containers are started automatically as part of the test process, so no separate setup is required for standard test execution.

## Development

To work on Minerva ETL locally, it is recommended to use a running Minerva database instance. A typical local development workflow looks like this:

1. Install the Minerva CLI
2. Start a test database using an example instance definition
3. Make changes in the relevant crate or service
4. Run the test suite to validate your changes

This setup helps ensure that changes can be validated in an environment that closely matches actual usage.

## Repository Structure

The repository is organized into several components, including:

```text
crates/
  admin-service/        Web API for administrative tasks
  cli/                  Command-line administration tools
  event-service/        Service to export trigger notifications to an HTTP endpoint
  integration-tests/    Contains end-to-end tests
  minerva/              Main crate with ETL code and database definitions
examples/     Example Minerva instance definitions
```

Additional crates and services in this repository support Minerva platform development and administration.

## Use Cases

Minerva ETL can be used for a range of tasks related to Minerva instance management, including:

- Defining and maintaining database schemas
- Managing Minerva instances through the CLI
- Integrating Minerva functionality into other Rust applications
- Supporting web-based administration workflows
- Setting up local development and test environments

## Notes

- The pg_query crate requires clang during compilation
- Tests are designed to run with automated container support
- Example instance definitions are provided to simplify local setup

## Contributing

Contributions are welcome. If you would like to improve the platform, fix bugs, or extend functionality, please follow the existing project structure and coding conventions.

When contributing:

- Keep changes focused and well scoped
- Add or update tests where relevant
- Make sure the full test suite passes before submitting changes

## License

[AGPL-3.0](LICENSE.md)
