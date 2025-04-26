# Plugin Objects

Handling objects

## Entity Types

| Name                 | Property       | Data Type | Socket Type |
|----------------------|----------------|-----------|-------------|
|                      |
| ObjectSetProperty    | object         | object    | input       |
|                      | property_name  | string    | input       |
|                      | property_value | any       | input       |
|                      | result         | object    | output      |
|                      |
| ObjectRemoveProperty | object         | object    | input       |
|                      | property_name  | string    | input       |
|                      | result         | object    | output      |
|                      | removed_value  | any       | output      |
|                      |
| ObjectGetProperty    | object         | object    | input       |
|                      | property_name  | string    | input       |
|                      | result         | any       | output      |
|                      |
| ObjectKeys           | object         | object    | input       |
|                      | keys           | array     | output      |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |
