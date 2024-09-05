# Plugin HTTP

This plugin provides a reactive http client integration. Using entities of type `http` requests via HTTP can be made.

With this plugin it's possible to integrate external services into your home automation.

## Type System

<img src="https://raw.githubusercontent.com/reactive-graph/plugins-core/main/docs/images/type_system.png">

## Entity Types

| Name    | Property         | Data Type | Socket Type |
|---------|------------------|-----------|-------------|
| Http    | url              | string    | input       |
|         | method           | string    | input       |
|         | request_headers  | object    | input       |
|         | payload          | object    | input       |
|         | response_headers | object    | output      |
|         | result           | object    | output      |
|         | status           | number    | output      |
| JsonRpc | url              | string    | input       |
|         | json_rpc_version | string    | input       |
|         | method           | string    | none        |
|         | params           | object    | input       |
|         | result           | object    | output      |
|         | error            | object    | output      |

## Entity Behaviours

| Name | Description                                                                                                                                                                         |
|------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Http | Sends a HTTP request with the given HTTP method to the given URL entity using the given headers and payload.<br/>The trigger is the property `payload` even if there is no payload. |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |
