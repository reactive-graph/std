# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module | Functionality                                                        | Tests                                                                                                                                                      |
|---------------------|--------|------------|----------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | Binary     | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-binary">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-binary)   |

### About this plugin

1. Loads binary data into a property. The mime type is automatically detected. Binary data is represented as BASE64 encoded data-url. Specification: https://fetch.spec.whatwg.org/#data-urls
2. Saves binary data, which is represented as a BASE64-encoded data-url, to a file.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/actions/workflow/status/inexorgame/inexor-rgf-plugin-binary/rust.yml">](https://github.com/inexorgame/inexor-rgf-plugin-binary/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-plugin-binary">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-plugin-binary">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-binary">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-binary)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-plugin-binary">](https://github.com/inexorgame/inexor-rgf-plugin-binary/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

#### Entity Types

| Name           | Property           | Data Type | Socket Type |
|----------------|--------------------|-----------|-------------|
| LoadBinaryData | filename           | string    | input       |
|                | data_url           | string    | output      |
| SaveBinaryData | filename           | string    | input       |
|                | data_url           | string    | input       |

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                              |           |                                                                   |
|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-binary/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
