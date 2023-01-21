# Plugin Binary

1. Loads binary data into a property. The mime type is automatically detected. Binary data is represented as BASE64 encoded data-url. Specification: https://fetch.spec.whatwg.org/#data-urls
2. Saves binary data, which is represented as a BASE64-encoded data-url, to a file.

## Entity Types

| Name           | Property           | Data Type | Socket Type |
|----------------|--------------------|-----------|-------------|
| LoadBinaryData | filename           | string    | input       |
|                | data_url           | string    | output      |
| SaveBinaryData | filename           | string    | input       |
|                | data_url           | string    | input       |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |
