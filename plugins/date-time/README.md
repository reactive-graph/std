# Plugin `date_time`

### About this plugin

This plugin provides the type system and behaviour for arithmetic operations.

### Entity Types

| Name           | Components | Properties | DataType | SocketType | Description                                           |
|----------------|------------|------------|----------|------------|-------------------------------------------------------|
||
| timestamp      | action     | trigger    | bool     | input      | Returns the current timestamp                         |
|                | action     | result     | number   | output     | The timestamp (UNIX epoch)                            |
||
| now            | action     | trigger    | bool     | input      | Returns the current date time                         |
|                | action     | result     | string   | output     | The current date time as ISO8601                      |
||
| datetime_plus  |            | datetime   | string   | input      | Adds the given duration from the given date time      |
|                |            | duration   | string   | input      | The duration to add (ISO8601 duration format)         |
|                |            | result     | string   | output     | The calculated date time as ISO8601                   |
||
| datetime_minus |            | datetime   | string   | input      | Subtracts the given duration from the given date time |
|                |            | duration   | string   | input      | The duration to subtract (ISO8601 duration format)    |
|                |            | result     | string   | output     | The calculated date time as ISO8601                   |
||
| datetime_diff  |            | datetime   | string   | input      | Calculates the difference of two datetimes            |
|                |            | datetime   | string   | input      |                                                       |
|                |            | result     | string   | output     | The calculated difference as ISO8601 duration format  |

### Crates

This plugin build on https://github.com/chronotope/chrono
