# Plugin Arithmetic

This plugin provides the type system and behaviours for arithmetic operations.

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
