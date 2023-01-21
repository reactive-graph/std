# Plugin Comparison

This plugin provides the type system and behaviour for comparisons of two inputs.

## Components

| Name            | Description                         | Property | Data Type   | Socket Type |
|-----------------|-------------------------------------|----------|-------------|-------------|
| comparison_gate | Compares two input values (lhs,rhs) | lhs      | any/depends | input       |
|                 |                                     | rhs      | any/depends | input       |
|                 |                                     | result   | any/depends | output      |

## Entity Types

| Name                   | Components      | Description                                                            |
|------------------------|-----------------|------------------------------------------------------------------------|
| equals                 | comparison_gate | Returns true, if lhs and rhs are equal                                 |
| greater_than           | comparison_gate | Returns true, if the value of lhs is greater than the value of rhs     |
| greater_than_or_equals | comparison_gate | Returns true, if value of lhs is greater than or equal to value of rhs |
| lower_than             | comparison_gate | Returns true, if value of lhs is lower than value of rhs               |
| lower_than_or_equals   | comparison_gate | Returns true, if value of lhs is lower than or equal to value of rhs   |
| not_equals             | comparison_gate | Returns true, if lhs and rhs are not equal                             |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |
