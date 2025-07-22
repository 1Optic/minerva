# Minerva ETL Platform

This project provides the Minerva ETL platform components:

1. Database schema definition.
2. Command line administration tools.
3. Library for interacting with Minerva instances.
4. Web service for Minerva instance administration.

## Prerequisites

- `clang` for the pg_query crate

## Installation

To install the administration command using cargo:

```sh
cargo install --path crates/cli
```

After that, you should have the Minerva CLI command available:

```sh
$ minerva --version
minerva 9.18.0
```

## Start Test Database

To develop Minerva instances, or work on the code in this project, you will
need a running Minerva database. You can start a Minerva database initialized
with a test instance using the CLI:

```sh
minerva start --with-definition examples/tiny_instance_v1
```

## Run Tests

Running the unit- and integration tests is as simple as running unit tests,
because test containers are automatically started in the tests.

```sh
cargo test
```
