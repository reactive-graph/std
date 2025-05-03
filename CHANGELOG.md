# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- [Book: std.reactive.graph.io](https://std.reactive.graph.io/)

### Changed

- Migrated to Rust Edition 2024
- Renamed repository `reactive-graph/plugins-core` -> `reactive-graph/std`
- Moved system library plugins to `reactive-graph/sys`
- Moved tooling library plugins to `reactive-graph/tooling`
- Moved network library plugins to `reactive-graph/net`
- Configure lints on workspace level

### Fixed

### Distribution

### Infrastructure

- CI: Update rust nightly toolchain to nightly-2025-03-14
- CI: Added GitHub Actions runner for arm64 ubuntu-22.04-arm
- CI: Generate release binaries and debian packages for arm64 using the arm64 runner
- CI: Automatically label pull requests
- CI: Automatically mark pull requests as stale
- CI: Automatically assign assignee and reviewers
- CI: Upgraded multiple actions
- CI: Synchronize labels from config file
- CI: Automatically merge successful dependabot PR's

## [0.9.1-21] - 2023-05-09

### Added

- Publish binaries for more targets, including Raspberry Pi
- Publish debian package for x86_64, aarch64 and armv7
