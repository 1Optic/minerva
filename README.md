# Minerva ETL Platform
Minerva ETL is the core platform for defining, managing, and operating Minerva instances. This repository contains the main components required for schema management, administration, development, and integration.

The platform supports Minerva environments through a combination of database definitions, command-line tooling, reusable libraries, and web-based administration services.

## Overview

This repository includes the following components:

- Database schema definitions for Minerva instances
- Command-line tools for administration and local development
- A Rust library for interacting with Minerva instances
- A web service for Minerva instance administration

Together, these components provide the foundation for working with Minerva in development, testing, and operational environments.

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

```bash
cargo install --path crates/cli
After installation, verify that the CLI is available:
minerva --version
Example output:
minerva 9.18.0

## Quick Start
To start a Minerva database initialized with a test instance, run:
minerva start --with-definition examples/tiny_instance_v1
This command starts a local Minerva instance using the example definition located in examples/tiny_instance_v1.
This is useful for:
•	Local development 
•	Testing changes 
•	Exploring the platform setup 
•	Verifying that your environment is configured correctly 

## Running Tests
Unit tests and integration tests can be run with:
cargo test
Test containers are started automatically as part of the test process, so no separate setup is required for standard test execution.

## Development
To work on Minerva ETL locally, it is recommended to use a running Minerva database instance. A typical local development workflow looks like this:
1.	Install the Minerva CLI 
2.	Start a test database using an example instance definition 
3.	Make changes in the relevant crate or service 
4.	Run the test suite to validate your changes 
This setup helps ensure that changes can be validated in an environment that closely matches actual usage.

## Repository Structure
The repository is organized into several components, including:
crates/
  cli/        Command-line administration tools
examples/     Example Minerva instance definitions
Additional crates and services in this repository support Minerva platform development and administration.

## Use Cases
Minerva ETL can be used for a range of tasks related to Minerva instance management, including:
•	Defining and maintaining database schemas 
•	Managing Minerva instances through the CLI 
•	Integrating Minerva functionality into other Rust applications 
•	Supporting web-based administration workflows 
•	Setting up local development and test environments 

## Notes
•	The pg_query crate requires clang during compilation 
•	Tests are designed to run with automated container support 
•	Example instance definitions are provided to simplify local setup 

## Contributing
Contributions are welcome. If you would like to improve the platform, fix bugs, or extend functionality, please follow the existing project structure and coding conventions.
When contributing:
•	Keep changes focused and well scoped 
•	Add or update tests where relevant 
•	Make sure the full test suite passes before submitting changes 
## License

