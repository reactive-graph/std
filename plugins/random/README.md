# Plugin Random

Generate random numbers

## Entity Types

| Name                     | Property | Data Type | Socket Type | Note      |
|--------------------------|----------|-----------|-------------|-----------|
|                          |
| RandomNumber             | trigger  | bool      | input       |           |
|                          | result   | number    | output      |           |
|                          |
| RandomIntegerWithinRange | trigger  | bool      | input       |           |
|                          | low      | number    | output      | Inclusive |
|                          | high     | number    | output      | Exclusive |
|                          | result   | number    | output      |           |

## TODO

- [ ] RandomNumberWithinRange (f64)
- [ ] RandomNumberNormalDistribution (f64)
- [ ] RandomString

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |
