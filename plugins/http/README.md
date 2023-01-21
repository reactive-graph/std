# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module | Functionality                                                        | Tests                                                                                                                                                |
|---------------------|--------|------------|----------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | HTTP       | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-http">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-http) |

### About this plugin

This plugin provides a reactive http client integration. Using entities of type `http` requests via HTTP can be made.

With this plugin it's possible to integrate external services into your home automation.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/actions/workflow/status/inexorgame/inexor-rgf-plugin-http/rust.yml">](https://github.com/inexorgame/inexor-rgf-plugin-http/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-plugin-http">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-plugin-http">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-http">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-http)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-plugin-http">](https://github.com/inexorgame/inexor-rgf-plugin-http/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

#### Type System

<img src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-http/main/docs/images/type_system.png">

#### Entity Types

| Name    | Property         | Data Type | Socket Type |
|---------|------------------|-----------|-------------|
| Http    | url              | string    | input       |
|         | method           | string    | input       |
|         | request_headers  | object    | input       |
|         | payload          | object    | input       |
|         | response_headers | object    | output      |
|         | result           | object    | output      |
|         | status           | number    | output      |
| JsonRpc | url              | string    | input       |
|         | json_rpc_version | string    | input       |
|         | method           | string    | none        |
|         | params           | object    | input       |
|         | result           | object    | output      |
|         | error            | object    | output      |

#### Entity Behaviours

| Name | Description                                                                                                                                                                           |
|------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Http | Sends a HTTP request with the given HTTP method to the given URL entity using the given headers and payload.<br/>The trigger is the property `payload` even if there is no payload.   |

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                            |           |                                                                   |
|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-http/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
