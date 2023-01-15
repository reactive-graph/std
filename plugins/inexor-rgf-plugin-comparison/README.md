# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module  | Functionality                                                        | Tests                                                                                                                                                            |
|---------------------|--------|-------------|----------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | Comparison  | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-comparison">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-comparison) |

### About this plugin

This plugin provides the type system and behaviour for comparisons of two inputs.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/inexorgame/inexor-rgf-plugin-comparison/Rust">](https://github.com/inexorgame/inexor-rgf-plugin-comparison/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-plugin-comparison">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-plugin-comparison">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-comparison">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-comparison)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-plugin-comparison">](https://github.com/inexorgame/inexor-rgf-plugin-comparison/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Components

| Name            | Description                         | Property | Data Type   | Socket Type |
|-----------------|-------------------------------------|----------|-------------|-------------|
| comparison_gate | Compares two input values (lhs,rhs) | lhs      | any/depends | input       |
|                 |                                     | rhs      | any/depends | input       |
|                 |                                     | result   | any/depends | output      |

#### Entity Types

| Name                   | Components      | Description                                                            |
|------------------------|-----------------|------------------------------------------------------------------------|
| equals                 | comparison_gate | Returns true, if lhs and rhs are equal                                 |
| greater_than           | comparison_gate | Returns true, if the value of lhs is greater than the value of rhs     |
| greater_than_or_equals | comparison_gate | Returns true, if value of lhs is greater than or equal to value of rhs |
| lower_than             | comparison_gate | Returns true, if value of lhs is lower than value of rhs               |
| lower_than_or_equals   | comparison_gate | Returns true, if value of lhs is lower than or equal to value of rhs   |
| not_equals             | comparison_gate | Returns true, if lhs and rhs are not equal                             |

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                                  |           |                                                                   |
|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-comparison/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
