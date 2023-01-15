# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module | Functionality                                                        | Tests                                                                                                                                                      |
|---------------------|--------|------------|----------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | Random     | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-random">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-random)   |

### About this plugin

Generate random numbers

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/actions/workflow/status/inexorgame/inexor-rgf-plugin-random/rust.yml">](https://github.com/inexorgame/inexor-rgf-plugin-random/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-plugin-random">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-plugin-random">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-random">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-random)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-plugin-random">](https://github.com/inexorgame/inexor-rgf-plugin-random/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Entity Types

| Name                     | Property | Data Type | Socket Type | Note      |
|--------------------------|----------|-----------|-------------|-----------|
|                          |
| RandomNumber             | trigger  | bool      | input       |           |
|                          | result   | number    | output      |           |
|                          |
| RandomIntegerWithinRange | trigger  | bool      | input       |           |
|                          | low      | number    | output      | Inclusive |
|                          | high     | number    | output      | Exclusive |
|                          | result   | number    | output      |           |

### TODO

* RandomNumberWithinRange (f64)
* RandomNumberNormalDistribution (f64)
* RandomString

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                              |           |                                                                   |
|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-random/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
