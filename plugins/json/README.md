| Project             | Module | Sub-Module | Functionality                                                        | Tests                                                                                                                                                  |
|---------------------|--------|------------|----------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | JSON       | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-json">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-json)   |

### About this plugin

Handling arrays and objects

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/actions/workflow/status/inexorgame/inexor-rgf-plugin-json/rust.yml">](https://github.com/inexorgame/inexor-rgf-plugin-json/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-plugin-json">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-plugin-json">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-json">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-json)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-plugin-json">](https://github.com/inexorgame/inexor-rgf-plugin-json/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Entity Types

| Name                 | Property          | Data Type | Socket Type |
|----------------------|-------------------|-----------|-------------|
|                      |
| ArrayPush            | array             | array     | input       |
|                      | to_be_added_value | any       | input       |
|                      | result            | array     | output      |
|                      |
| ArrayPop             | array             | array     | input       |
|                      | result            | array     | output      |
|                      | removed_value     | any       | input       |
|                      |
| ArrayGetByIndex      | array             | array     | input       |
|                      | index             | number    | output      |
|                      | result            | any       | output      |
|                      |
| ArrayLength          | array             | array     | input       |
|                      | length            | number    | output      |
|                      |
| ArrayReverse         | array             | array     | input       |
|                      | result            | array     | output      |
|                      |
| ArrayContains        | array             | array     | input       |
|                      | search            | any       | input       |
|                      | result            | bool      | output      |
|                      |
| ObjectSetProperty    | object            | object    | input       |
|                      | property_name     | string    | input       |
|                      | property_value    | any       | input       |
|                      | result            | object    | output      |
|                      |
| ObjectRemoveProperty | object            | object    | input       |
|                      | property_name     | string    | input       |
|                      | result            | object    | output      |
|                      | removed_value     | any       | output      |
|                      |
| ObjectGetProperty    | object            | object    | input       |
|                      | property_name     | string    | input       |
|                      | result            | any       | output      |
|                      |
| ObjectKeys           | object            | object    | input       |
|                      | keys              | array     | output      |

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                            |           |                                                                   |
|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-json/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
