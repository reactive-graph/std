# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module | Functionality                                                        | Tests                                                                                                                                                      |
|---------------------|--------|------------|----------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | Logical    | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-logical">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-logical) |

### About this plugin

This plugin provides the type system and behaviours for logical operations.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/inexorgame/inexor-rgf-plugin-logical/Rust">](https://github.com/inexorgame/inexor-rgf-plugin-logical/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-plugin-logical">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-plugin-logical">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-logical">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-logical)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-plugin-logical">](https://github.com/inexorgame/inexor-rgf-plugin-logical/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

#### Components

| Name              | Description | Properties | Data Type | Socket Type |
|-------------------|-------------|------------|-----------|-------------|
| logical_gate      |             | lhs        | bool      | input       |
|                   |             | rhs        | bool      | input       |
|                   |             | result     | bool      | output      |
| logical_operation |             | lhs        | bool      | input       |
|                   |             | result     | bool      | output      |
| condition         |             | condition  | bool      | input       |
|                   |             | result     | any       | output      |

#### Entity Types

| Name         | Components        | Description                                                                                                                              |
|--------------|-------------------|------------------------------------------------------------------------------------------------------------------------------------------|
| and          | logical_gate      |                                                                                                                                          |
| nand         | logical_gate      |                                                                                                                                          |
| nor          | logical_gate      |                                                                                                                                          |
| not          | logical_operation |                                                                                                                                          |
| or           | logical_gate      |                                                                                                                                          |
| xnor         | logical_gate      |                                                                                                                                          |
| xor          | logical_gate      |                                                                                                                                          |
| trigger      | condition         | <ul><li>If condition is true, propagate payload to result</li></ul>                                                                      |
| if_then_else | condition         | <ul><li>If condition is true, propagate then_payload to result</li><li>If condition is false, propagate else_payload to result</li></ul> |

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                               |           |                                                                   |
|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-logical/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
