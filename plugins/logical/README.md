# Plugin Logical

This plugin provides the type system and behaviours for logical operations.

## Components

| Name              | Description | Properties | Data Type | Socket Type |
|-------------------|-------------|------------|-----------|-------------|
| logical_gate      |             | lhs        | bool      | input       |
|                   |             | rhs        | bool      | input       |
|                   |             | result     | bool      | output      |
| logical_operation |             | lhs        | bool      | input       |
|                   |             | result     | bool      | output      |
| condition         |             | condition  | bool      | input       |
|                   |             | result     | any       | output      |

## Entity Types

| Name         | Components        | Description                                                                                                                              |
|--------------|-------------------|------------------------------------------------------------------------------------------------------------------------------------------|
| and          | logical_gate      |                                                                                                                                          |
| nand         | logical_gate      |                                                                                                                                          |
| nor          | logical_gate      |                                                                                                                                          |
| not          | logical_operation |                                                                                                                                          |
| or           | logical_gate      |                                                                                                                                          |
| xnor         | logical_gate      |                                                                                                                                          |
| xor          | logical_gate      |                                                                                                                                          |
| trigger      | condition         | <ul><li>If condition is true, propagate payload to result</li></ul>                                                                      |
| if_then_else | condition         | <ul><li>If condition is true, propagate then_payload to result</li><li>If condition is false, propagate else_payload to result</li></ul> |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |
