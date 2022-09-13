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

#### Components

| Name              | Property | Data Type | Socket Type |
|-------------------|----------|-----------|-------------|
| string_operation  | lhs      | string    | input       |
|                   | result   | string    | output      |
| string_gate       | lhs      | string    | input       |
|                   | rhs      | string    | input       |
|                   | result   | string    | output      |
| string_comparison | lhs      | string    | input       |
|                   | rhs      | string    | input       |
|                   | result   | bool      | output      |

#### Entity Types / Behaviours

| Name               | Component         | Description                                                                                             |
|--------------------|-------------------|---------------------------------------------------------------------------------------------------------|
| camel_case         | string_operation  | Converts the input to camel case                                                                        |
| capitalize         | string_operation  | Converts the first character of the input to upper case and convert the rest of the input to lower case |
| chop_after         | string_operation  | Returns everything after the given search                                                               |
| chop_after_last    | string_operation  | Returns everything after the last given search                                                          |
| chop_before        | string_operation  | Returns everything before the given search                                                              |
| chop_before_last   | string_operation  | Returns everything before the last given search                                                         |
| chop_remove_prefix | string_operation  | Extracts the prefix from the input                                                                      |
| chop_remove_suffix | string_operation  | Extracts the suffix from the input                                                                      |
| decapitalize       | string_operation  | Converts the first character of the input to lower case and convert the rest of the input to lower case |
| concat             | string_gate       | Concatenate lhs with rhs                                                                                |
| contains           | string_comparison | Returns true, if lhs contains rhs                                                                       |
| ends_with          | string_comparison | Returns true, if lhs ends with rhs                                                                      |
| kebab_case         | string_operation  | Converts the input to kebab case                                                                        |
| lowercase          | string_operation  | Converts the input to lower case                                                                        |
| lower_first        | string_operation  | Converts the first character of the input to lower case                                                 |
| pascal_case        | string_operation  | Converts the input to pascal case                                                                       |
| starts_with        | string_comparison | Returns true, if lhs starts with rhs                                                                    |
| shouty_kebab_case  | string_operation  | Converts the input to SHOUTY kebab case                                                                 |
| shouty_snake_case  | string_operation  | Converts the input to SHOUTY snake case                                                                 |
| snake_case         | string_operation  | Converts the input to snake case                                                                        |
| swap_case          | string_operation  | Converts the input to swap case                                                                         |
| title_case         | string_operation  | Converts the input to title case                                                                        |
| train_case         | string_operation  | Converts the input to train case                                                                        |
| trim               | string_operation  | Removes whitespace at the beginning and end of a string                                                 |
| trim_start         | string_operation  | Removes whitespace at the beginning of a string                                                         |
| trim_end           | string_operation  | Removes whitespace at the end of a string                                                               |
| uppercase          | string_operation  | Converts the input to upper case                                                                        |
| upper_first        | string_operation  | Converts the first character of the input to upper case                                                 |

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
