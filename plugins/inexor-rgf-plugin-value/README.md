# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module | Functionality                                                        | Tests                                                                                                                                                  |
|---------------------|--------|------------|----------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | Value      | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-value">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-value) |

### About this plugin

This plugin provides the type system for pure-value entities. These are entities which stores one single value.
There are entity types for each data type.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/inexorgame/inexor-rgf-plugin-value/Rust">](https://github.com/inexorgame/inexor-rgf-plugin-value/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-plugin-value">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-plugin-value">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-value">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-value)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-plugin-value">](https://github.com/inexorgame/inexor-rgf-plugin-value/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

#### Components

| *Component*            | *Properties* | *Data Type* | *Socket Type* | Description                           |
|------------------------|--------------|-------------|---------------|---------------------------------------|
|                        |
| value_boolean          | value        | boolean     | output        | A boolean value                       |
| value_number           | value        | number      | output        | A numeric value                       |
| value_string           | value        | string      | output        | A string value                        |
| value_array            | value        | array       | output        | A array value                         |
| value_object           | value        | object      | output        | A object value                        | 
|                        |
| state_boolean          | state        | boolean     | none          | A boolean state                       |
|                        | set_state    | boolean     | input         |
| state_number           | state        | number      | none          | A numeric state                       |
|                        | set_state    | number      | input         |
| state_string           | state        | string      | none          | A string state                        |
|                        | set_state    | string      | input         |
| state_array            | state        | array       | none          | A array state                         |
|                        | set_state    | array       | input         |
| state_object           | state        | object      | none          | A object state                        |
|                        | set_state    | object      | input         |
|                        |
| value_debugger_debug   |              |             |               | Debugger for values (log level debug) |
| value_debugger_trace   |              |             |               | Debugger for values (log level trace) |
|                        |
| state_debugger_debug   |              |             |               | Debugger for states (log level debug) |
| state_debugger_trace   |              |             |               | Debugger for states (log level trace) |

#### Entity Types

| Name          | Components    | Description     |
|---------------|---------------|-----------------|
|               |
| value_array   | value_array   | A array value   |
| value_boolean | value_boolean | A boolean value |
| value_number  | value_number  | A numeric value |
| value_string  | value_string  | A string value  |
| value_object  | value_object  | A object value  |
|               |
| state_array   | value_array   | A array state   |
|               | state_array   |                 |
| state_boolean | value_boolean | A boolean state |
|               | state_boolean |                 |
| state_number  | value_number  | A numeric state |
|               | state_boolean |                 |
| state_string  | value_string  | A string state  |
|               | state_boolean |                 |
| state_object  | value_object  | A object state  | 
|               | state_boolean |                 |

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                             |           |                                                                   |
|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-value/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
