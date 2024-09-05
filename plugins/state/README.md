# Plugin `reactive-graph-plugin-state`

### About this plugin

#### Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

#### Components

| *Component*          | *Properties* | *Data Type* | *Socket Type* | Description                           |
|----------------------|--------------|-------------|---------------|---------------------------------------|
| state_boolean        | state        | boolean     | none          | A boolean state                       |
|                      | set_state    | boolean     | input         |
| state_number         | state        | number      | none          | A numeric state                       |
|                      | set_state    | number      | input         |
| state_string         | state        | string      | none          | A string state                        |
|                      | set_state    | string      | input         |
| state_array          | state        | array       | none          | A array state                         |
|                      | set_state    | array       | input         |
| state_object         | state        | object      | none          | A object state                        |
|                      | set_state    | object      | input         |
|                      |
| state_debugger_debug |              |             |               | Debugger for states (log level debug) |
| state_debugger_trace |              |             |               | Debugger for states (log level trace) |

#### Entity Types

| Name          | Components    | Description     |
|---------------|---------------|-----------------|
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
