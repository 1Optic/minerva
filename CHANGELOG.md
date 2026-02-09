# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [9.43.0] - 2026-02-09

### Added

- admin-service: Allow putting threshold values in template parameters
- cli: Add pgpass file support
- design: Add multi-stage relation materialization
- lib: Improve entity id and alias handling

### Fixed

- docs: Fix mdbook build and warnings
- lib: Fix generated aggregation durations
- lib: Fix cascade deletion of trigger-tables

## [9.42.0] - 2026-01-14

### Added

- CLI: Add logging of changes that can be reverted when running update

## [9.41.0] - 2026-01-12

### Fixed

- Library: Fixed ambiguous change naming for attribute store and notification store attribute addition

## [9.40.0] - 2026-01-09

### Fixed

- CLI: Fixed unneeded requirement of MINERVA_INSTANCE_ROOT variable when updating from diff

### Changed

- CLI: Minimize information in diff JSON to improve readability

## [9.39.0] - 2026-01-08

### Changed

- CLI: Serialize diff JSON as array instead of multi-object stream

## [9.38.0] - 2026-01-08

### Added

- CLI: Add option to load update changes from JSON diff file

## [9.37.3] - 2025-12-18

### Fixed

- CLI: Ignore trend store part deletions when --ignore-deletions is set

## [9.37.2] - 2025-12-17

### Changed

- Library: No longer populate all default alias values when enabling it for a trend store part

## [9.37.1] - 2025-12-17

### Fixed

- CLI: Fixed updating of primary alias that was not properly commited

## [9.37.0] - 2025-12-17

### Added

- CLI: Added updating of primary alias configuration of entity types

### Fixed

- CLI: Fixed creation of specified entity types in initialize command

## [9.36.0] - 2025-11-06

### Added

- API: Add trigger ID support
- Trigger: Extend templates to be able to handle avg, min, max, sum... over a past time period

### Fixed

- DB: Fix migration filename

## [9.35.0] - 2025-11-06

### Added

- CLI: Get attribute store from definition

### Changed

- Add PostGIS to instance tester container image

## [9.34.0] - 2025-08-26

### Added

- CLI: Added support for ignoring trend data type changes using instance config file.

## [9.33.1] - 2025-08-22

### Fixed

- Library: Fixed loading of 'boolean' and 'numeric[]' trend data types
- CLI: Fixed trend value information table view for trend delete changes

## [9.33.0] - 2025-08-12

### Added

- DB: Support timestamptz in data_type_order function

### Changed

- Tests: Use custom test harness to reuse Minerva cluster for multiple integration tests
- DB: Store trend statistics in separate table 'table_trend_statistics'

### Fixed

- Library: Fix data type mismatch detection when loading trend data using copy from

## [9.32.0] - 2025-07-22

### Added

- CLI: Support loading environment variables from a file
- CLI: Support detection of removed trend store parts in diff and update

### Fixed

- CLI/Library: Compare attribute store entity type name case insensitive in diff

## [9.31.0] - 2025-07-16

### Fixed

- Library: Fixed handling of datatype mismatches
- Library: Fixed issue with duplicate default alias data injection

## [9.30.0] - 2025-07-07

### Added

- CLI: Added list-chunks subcommand to check what trend materializations are pending

### Removed

- DB: Removed the 'has_primary_alias' generated column
- DB: Unused functions with logic that is now implemented in the CLI client/library

## [9.29.0] - 2025-07-04

### Added

- CLI/Library: Added support for compacting attribute stores with bigint type id column

## [9.28.0] - 2025-06-13

### Added

- CLI/Library: Added support for defining entity types in YAML/JSON
- CLI: Added support for generating subgraph of specific object with dependencies or dependees
- CLI: Added support for generating update plan without altering the database

## [9.27.1] - 2025-05-30

### Removed

- DB Schema: Removed broken notification generation functions.

## [9.27.0] - 2025-05-27

### Fixed

- Library: Fixed bug in trend materialization logic update that caused errors
  due to multiple attempts of removing views and functions.

### Added

- CLI/Library: Added option to show detailed diff for trend extra data change
- CLI/Library: Added option to show detailed diff for trend materialization function SQL

### Changed

- Library: Use specific error types for trend data storage functions to allow handling of data type mismatches
- Integration tests: Restructured integration tests so that they are compiled separately.

## [9.26.1] - 2025-04-25

### Fixed

- CLI/Library: Fixed migration issue with notification functions
- CLI/Library: Fixed updating of sources and fingerprint functions of materializations.

## [9.26.0] - 2025-03-27

### Added

- CLI: Added support for diffing of trend materialization logic
- CLI: Added support for suppressing aggregation generation for specific relations
- Service: Added 'version' endpoint

## [9.25.0] - 2025-03-24

### Fixed

- CLI: Exit with error failure code on differences with instance definition.
- Library: Use Citus sequential mode for attribute store changes to prevent
  Citus related errors.

### Added

- CLI: Added subcommand to generate trend data dependency graph.
- DB Schema: Added data column for triggers and removed old details column.

### Changed

- CLI: Report missing relation definition views as warnings and do not return
  error exit code.
- CLI: Enumerate changes during instance update as progress indicator.

## [9.24.0] - 2025-03-10

### Changed

- Library: Added retention_period to the trend store definition instead of
  always using a default.
- DB Schema: Made trend function materialization more robust by only
  materializing columns that are defined in the materialization logic.

## [9.23.0] - 2025-03-04

### Added

- CLI: Add option for formatting diff as JSON

## [9.22.0] - 2025-03-03

### Changed

- Library: Use Client instead of Transaction in Change::apply
- Library: Create trend store part including all trends

### Fixed

- Service: Fixed rigts issues with creation of user defined triggers

## [9.21.3] - 2025-02-27

### Fixed

- CLI: Propagate errors for relation and virtual entity materializations

## [9.21.2] - 2025-02-27

### Fixed

- CLI: Exit with error failure code on errors
- Service: Show id in a separate field when creating an entity set in backend
- Database: Fix permissions for entity set related relation tables

## [9.21.1] - 2025-02-25

### Fixed

- Fixed addition of samples trend for generated entity aggregations

## [9.21.0] - 2025-02-18

### Added

- CLI: Option for omitting deletions in diff

## [9.20.2] - 2025-02-12

### Fixed

- Materialization: Fixed selection of materializations for execution

## [9.20.1] - 2025-02-12

### Fixed

- Service: Fixed parameter name to match API specification

## [9.20.0] - 2025-02-12

### Added

- Service: Add endpoints for template-based trigger management

## [9.19.0] - 2025-02-11

### Added

- CLI: Option for omitting trend data type changes in diff
- CLI: Option for omitting trend extra data changes in diff

### Changed

- CLI: Show details for trend removal, addition and data type changes in diff

## [9.18.1] - 2025-02-11

### Fixed

- Fixed datatype mismatch when loading trend materializations from the
  database

## [9.18.0] - 2025-01-29

### Added

- Added attribute materialization initialization
- Added support for attribute materialization

## [9.17.1] - 2025-01-20

### Fixed

- Fix bug in curr-ptr materialization query

## [9.17.0] - 2025-01-20

### Fixed

- Fixed Citus initialization in test databases in test clusters
- Fixed default aggregation generation

### Added

- Added support for Minerva instance config file
- Added support for specifying custom Docker image for test clusters
- Added support for materializing attribute materialized view after curr-ptr materialization
- Added separate materialization stability delay for historic data

### Changed

- Specifying aggregation hints is now done in Minerva instance config file

## [9.16.2] - 2024-12-30

### Fixed

- DB Schema: Fixed name of V2 migration

## [9.16.1] - 2024-12-30

### Fixed

- DB Schema: Restore V1 migration to original state

## [9.16.0] - 2024-12-30

### Added

- CLI: Option to run sanity checks on trend materialization creation
- CLI: New subcommand for running sanity checks on all trend materializations
- CLI: New subcommand for removing trend materializations

## [9.15.0] - 2024-12-24

### Changed

- Library: Added support for specifying which Docker image to use for a test cluster

## [9.14.1] - 2024-12-12

### Fixed

- CLI: Fix bug in materialize service query for materializations to run

## [9.14.0] - 2024-12-11

### Changed

- Docker: Use CLI as Docker image entrypoint

## [9.13.0] - 2024-12-10

### Added

- CLI: New subcommand for the materialization service

## [9.12.0] - 2024-11-28

### Added

- CLI: New subcommand for database schema migrations
- CLI: New subcommand for attribute store compacting

### Changed

- All logic for compacting is now implemented in the client instead of in
  database functions and views.

## [9.11.0] - 2024-11-26

### Added

- Design: Add default alias column design
- Service: Add endpoint to delete entity sets by id

### Changed

- Upgrade dependencies

### Fixed

- Handle SQL inputs securely

## [9.10.0] - 2024-11-13

### Changed

- CLI: The command for materializing virtual entities now accepts multiple type
  names and materializes all when none is specified

## [9.9.0] - 2024-11-11

### Added

- CLI: Added command for materializing virtual entities
- Service: Add endpoint to delete entity sets

## [9.8.0] - 2024-10-30

### Added

- Added default aggregation generation command
- Added command for converting old partitions to columnar storage
- Added support for updating triggers in the API

### Changed

- Improved error messages for trigger management API endpoints

## [9.7.0] - 2024-10-10

### Changed

- Use Id instead of name for entity set identification in API

### Fixed

- Fix passing of MINERVA_INSTANCE_ROOT to child processes for initialization

## [9.6.2] - 2024-09-03

### Fixed

- Fix inclusion of new attribute stores for curr-ptr materialization.

## [9.6.1] - 2024-08-27

### Fixed

- Fix handling duplicate key errors when storing trend data.

## [9.6.0] - 2024-08-27

### Added

- Added cache-less entity mapper DbEntityMapping.

## [9.5.0] - 2024-08-22

### Changed

- Improved error reporting on trend materialization management commands.
- Proper table rendering for materialization list.

## [9.4.0] - 2024-08-22

### Added

- Added subcommand to materialize relations.

## [9.3.0] - 2024-08-22

### Added

- Added subcommand to update relation definitions.

## [9.2.0] - 2024-08-21

### Added

- Added subcommand to materialize attribute curr-ptr tables.

## [9.1.1] - 2024-08-21

### Fixed

- Fixed updating of the attribute store modified timestamp on data loading.

## [9.1.0] - 2024-08-13

### Changed

- Update rust crate testcontainers to 0.21
- Update rust crate actix-web to v4.9.0
- Update rust crate tempfile to v3.12.0

## [9.0.0] - 2024-07-26

### Changed

- Inject entity mapping logic in attribute and trend storage.

## [8.2.0] - 2024-07-25

### Added

- Added function to clear entity mapping cache.

## [8.1.1] - 2024-07-25

### Changed

- Fixed bug in view-materialization insert query building.

## [8.1.0] - 2024-07-25

### Changed

- Use fixed-size entity mapping cache to prevent unbounded memory usage.

## [8.0.0] - 2024-07-24

### Changed

- AttributeDataRow now includes timestamp field

[8.0.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/7.7.1...8.0.0
[8.1.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/8.0.0...8.1.0
[8.1.1]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/8.1.0...8.1.1
[8.2.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/8.1.1...8.2.0
[9.0.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/8.2.0...9.0.0
[9.1.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.0.0...9.1.0
[9.1.1]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.1.0...9.1.1
[9.2.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.1.1...9.2.0
[9.3.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.2.0...9.3.0
[9.4.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.3.0...9.4.0
[9.5.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.4.0...9.5.0
[9.6.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.5.0...9.6.0
[9.6.1]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.6.0...9.6.1
[9.6.2]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.6.1...9.6.2
[9.7.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.6.2...9.7.0
[9.8.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.7.0...9.8.0
[9.9.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.8.0...9.9.0
[9.10.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.9.0...9.10.0
[9.11.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.10.0...9.11.0
[9.12.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.11.0...9.12.0
[9.13.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.12.0...9.13.0
[9.14.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.13.0...9.14.0
[9.14.1]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.14.0...9.14.1
[9.15.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.14.1...9.15.0
[9.16.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.15.0...9.16.0
[9.16.1]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.16.0...9.16.1
[9.16.2]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.16.1...9.16.2
[9.17.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.16.2...9.17.0
[9.17.1]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.17.0...9.17.1
[9.18.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.17.1...9.18.0
[9.18.1]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.18.0...9.18.1
[9.19.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.18.1...9.19.0
[9.20.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.19.0...9.20.0
[9.20.1]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.20.0...9.20.1
[9.20.2]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.20.1...9.20.2
[9.21.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.20.2...9.21.0
[9.21.1]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.21.0...9.21.1
[9.21.2]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.21.1...9.21.2
[9.21.3]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.21.2...9.21.3
[9.22.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.21.3...9.22.0
[9.23.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.22.0...9.23.0
[9.24.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.23.0...9.24.0
[9.25.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.24.0...9.25.0
[9.26.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.25.0...9.26.0
[9.26.1]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.26.0...9.26.1
[9.27.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.26.1...9.27.0
[9.27.1]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.27.0...9.27.1
[9.28.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.27.1...9.28.0
[9.29.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.28.0...9.29.0
[9.30.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.29.0...9.30.0
[9.31.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.30.0...9.31.0
[9.32.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.31.0...9.32.0
[9.33.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.32.0...9.33.0
[9.33.1]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.33.0...9.33.1
[9.34.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.33.1...9.34.0
[9.35.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.34.0...9.35.0
[9.36.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.35.0...9.36.0
[9.37.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.36.0...9.37.0
[9.37.1]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.37.0...9.37.1
[9.37.2]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.37.1...9.37.2
[9.37.3]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.37.2...9.37.3
[9.38.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.37.3...9.38.0
[9.39.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.38.0...9.39.0
[9.40.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.39.1...9.40.0
[9.41.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.40.0...9.41.0
[9.42.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.41.0...9.42.0
[9.43.0]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.42.0...9.43.0
[Unreleased]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.43.0...HEAD
