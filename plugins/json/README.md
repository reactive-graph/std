# Plugin JSON

Handling arrays and objects

## Entity Types

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
| ArrayLength          | array             | array     | input       |
|                      | length            | number    | output      |
|                      |
| ArrayReverse         | array             | array     | input       |
|                      | result            | array     | output      |
|                      |
| ArrayContains        | array             | array     | input       |
|                      | search            | any       | input       |
|                      | result            | bool      | output      |
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
|                      |
| ObjectKeys           | object            | object    | input       |
|                      | keys              | array     | output      |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |
