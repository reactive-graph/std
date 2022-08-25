# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module | Functionality                                                        | Tests                                                                                                                                                      |
|---------------------|--------|------------|----------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | String     | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-string">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-string)   |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-string/main/docs/images/inexor_2.png">
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

String Handling

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/inexorgame/inexor-rgf-plugin-string/Rust">](https://github.com/inexorgame/inexor-rgf-plugin-string/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-plugin-string">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-plugin-string">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-string">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-string)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-plugin-string">](https://github.com/inexorgame/inexor-rgf-plugin-string/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Type System

<img src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-string/main/docs/images/type_system.png">

#### Components

| Name             | Property | Data Type | Socket Type |
|------------------|----------|-----------|-------------|
| StringOperation  | lhs      | string    | input       |
|                  | result   | string    | output      |
| StringGate       | lhs      | string    | input       |
|                  | rhs      | string    | input       |
|                  | result   | string    | output      |
| StringComparison | lhs      | string    | input       |
|                  | rhs      | string    | input       |
|                  | result   | bool      | output      |

#### Entity Types / Behaviours

| Name       | Component        | Description                                              |
|------------|------------------|----------------------------------------------------------|
| Trim       | StringOperation  | Removes whitespace at the beginning and end of a string  |
| TrimStart  | StringOperation  | Removes whitespace at the beginning of a string          |
| TrimEnd    | StringOperation  | Removes whitespace at the end of a string                |
| Uppercase  | StringOperation  |                                                          |
| Lowercase  | StringOperation  |                                                          |
| StartsWith | StringComparison |                                                          |
| EndsWith   | StringComparison |                                                          |
| Contains   | StringComparison |                                                          |

### TODO

| Name       | Component        | Description                                              |
|------------|------------------|----------------------------------------------------------|
| Split      |                  | lhs (str), rhs (str) -> result (array of str)            |
| Replace    |                  | lhs (str), search (str), replace (str) -> result (str)   |
| Chars      |                  | lhs (str) -> result (array of str)                       |
| Len        |                  | lhs (str) -> result (f64)                                |
| Lines      |                  | lhs (str) -> result (array of str)                       |

length (str: str) => number
insert (str: str, to_be_inserted: str, at_pos: number) => str
replace (str: str, search: str, replace: str) => str
split (str: str, pos: number) => (str, str)
is_empty (str: str) => bool
substring (str: str, start: number, end: number) => str
to_lines (str: str) => arr(str)
to_chars (str: str) => arr(str)
split_whitespace (str: str) => arr(str)
find (str: str) => number

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                              |           |                                                                   |
|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-string/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
