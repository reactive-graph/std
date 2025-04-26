# Plugin `reactive-graph-std-value`

### About this plugin

This plugin provides the type system for pure-value entities. These are entities which stores one single value.
There are entity types for each data type.

#### Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

#### Components

| *Component*          | *Properties* | *Data Type* | *Socket Type* | Description                           |
|----------------------|--------------|-------------|---------------|---------------------------------------|
| value_boolean        | value        | boolean     | output        | A boolean value                       |
| value_number         | value        | number      | output        | A numeric value                       |
| value_string         | value        | string      | output        | A string value                        |
| value_array          | value        | array       | output        | A array value                         |
| value_object         | value        | object      | output        | A object value                        | 
|                      |
| value_debugger_debug |              |             |               | Debugger for values (log level debug) |
| value_debugger_trace |              |             |               | Debugger for values (log level trace) |

#### Entity Types

| Name          | Components    | Description     |
|---------------|---------------|-----------------|
| value_array   | value_array   | A array value   |
| value_boolean | value_boolean | A boolean value |
| value_number  | value_number  | A numeric value |
| value_string  | value_string  | A string value  |
| value_object  | value_object  | A object value  |
