# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module | Functionality                                                        | Tests                                                                                                                                                                |
|---------------------|--------|------------|----------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | JSON       | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-json">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-json) |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-json/main/docs/images/inexor_2.png">
</a>

* Inexor will be a new first-person shooter game which is based on a new octree-based game engine.
* Inexor focuses on classic gameplay as we've seen in Cube2 or the Quake series.
* Inexor will be written from ground up new in C++17 and Rust.
* You can contribute anything you want: code, content, ideas..
* Inexor and all its content is 100% open source!

### About Inexor Reactive Graph Flow

The Inexor Reactive Graph Flow (RGF) manages reactive flows based on a graph database. The main interface is GraphQL.

* Semantic: Graph database with entities and relationships as first class citizens
* Reactive: entities and relationships are/can be reactive: If the input has been altered the entity processes its new state
* Interoperable: Use GraphQL for queries and mutations
* Extendable: Built in type system: components, entity types and relation types
* Memory efficient: Rust
* Fast: Rust
* Secure: Rust

### About this plugin

Handling arrays and objects

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/inexor-rgf-plugin-json/Rust">](https://github.com/aschaeffer/inexor-rgf-plugin-json/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/aschaeffer/inexor-rgf-plugin-json">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/inexor-rgf-plugin-json">]()
[<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-json">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-json)

[<img src="https://img.shields.io/github/license/aschaeffer/inexor-rgf-plugin-json">](https://github.com/aschaeffer/inexor-rgf-plugin-json/blob/main/LICENSE)
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

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                               |           |                                                                   |
|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-logical/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
