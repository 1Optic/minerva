# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.21.1...HEAD
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
