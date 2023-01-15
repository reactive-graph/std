# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module | Functionality                                                        | Tests                                                                                                                                                            |
|---------------------|--------|------------|----------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | Arithmetic | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-arithmetic">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-arithmetic) |

### About this plugin

This plugin provides the type system and behaviour for arithmetic operations.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/actions/workflow/status/inexorgame/inexor-rgf-plugin-arithmetic/rust.yml">](https://github.com/inexorgame/inexor-rgf-plugin-arithmetic/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-plugin-arithmetic">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-plugin-arithmetic">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-arithmetic">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-arithmetic)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-plugin-arithmetic">](https://github.com/inexorgame/inexor-rgf-plugin-arithmetic/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Components

| Name                 | Property | Data Type | Socket Type | Description |
|----------------------|----------|-----------|-------------|-------------|
| arithmetic_operation | lhs      | number    | input       |             |
|                      | result   | number    | output      |             |
|
| arithmetic_gate      | lhs      | number    | input       |             |
|                      | rhs      | number    | input       |             |
|                      | result   | number    | output      |             |

#### Entity Types

| Name      | Components           | Description                |
|-----------|----------------------|----------------------------|
| add       | arithmetic_gate      | Addition                   |
| counter   | action               | Increases the counter by 1 |
| decrement | arithmetic_operation | Decrements the value by 1  |
| div       | arithmetic_gate      | Division                   |
| increment | arithmetic_operation | Increments the value by 1  |
| max       | arithmetic_gate      | Max value                  |
| min       | arithmetic_gate      | Min value                  |
| mod       | arithmetic_gate      | Modulo                     |
| mul       | arithmetic_gate      | Multiplication             |
| sub       | arithmetic_gate      | Subtraction                |

#### Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                                  |           |                                                                   |
|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-arithmetic/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
