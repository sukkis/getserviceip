# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/sukkis/getserviceip/compare/v0.1.0...v0.1.1) - 2024-09-22

### Fixed

- allow connections from other hosts

### Other

- Merge pull request [#14](https://github.com/sukkis/getserviceip/pull/14) from sukkis/allow-connections-to-server

## [0.1.0](https://github.com/sukkis/getserviceip/releases/tag/v0.1.0) - 2024-09-21

### Added

- new endpoint called host_details, to get information on one host
- take release-plz into use
- list_all endpoint

### Fixed

- change the name of the master branch
- make sure tests/common/mod.rs is included in the published package
- change the include so that LICENSE and README are also there
- change include clause
- refactor AppState for user friendliness
- store state
- needless borrow removed
- refactor ip endpoint test
- cargo fmt changes
- initial pipeline

### Other

- update README
- new endpoint described
- Merge pull request [#2](https://github.com/sukkis/getserviceip/pull/2) from sukkis/initial-docs
- add initial README
- Initial commit
