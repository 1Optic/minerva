# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://gitlab.1optic.io/hitc/Minerva/minerva/-/compare/9.7.0...HEAD
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
