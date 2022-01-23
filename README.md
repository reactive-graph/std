# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module | Functionality                                                        | Tests                                                                                                                                                      |
|---------------------|--------|------------|----------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | Numeric    | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-numeric">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-numeric) |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-numeric/main/docs/images/inexor_2.png">
</a>

* Inexor will be a new first-person shooter game which is based on a new octree-based game engine.
* Inexor focuses on classic gameplay as we've seen in Cube2 or the Quake series.
* Inexor will be written from ground up new in C++17 and Rust.
* You can contribute anything you want: code, content, ideas..
* Inexor and all its content is 100% open source!

### About Inexor Reactive Graph Flow

The Inexor Reactive Graph Flow (RGF) manages reactive flows based on a graph database. The main interface is GraphQL.

* Semantic: Graph database with entities and relationships as first class citizens
* Reactive: entities and relationships are/can be reactive: If the input has been altered the entity processes its new state
* Interoperable: Use GraphQL for queries and mutations
* Extendable: Built in type system: components, entity types and relation types
* Memory efficient: Rust
* Fast: Rust
* Secure: Rust

### About this plugin

This plugin provides the type system and behaviour for numeric operations.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/inexor-rgf-plugin-numeric/Rust">](https://github.com/aschaeffer/inexor-rgf-plugin-numeric/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/aschaeffer/inexor-rgf-plugin-numeric">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/inexor-rgf-plugin-numeric">]()
[<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-numeric">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-numeric)

[<img src="https://img.shields.io/github/license/aschaeffer/inexor-rgf-plugin-numeric">](https://github.com/aschaeffer/inexor-rgf-plugin-numeric/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

#### Components

| Name              | Description | Input Properties | Output Properties |
|-------------------|-------------|------------------|-------------------|
| numeric_gate      |             | lhs<br>rhs       | result            |
| numeric_operation |             | lhs              | result            |

#### Entity Types

| Name       | Components        | Description                                                                                    |
|------------|-------------------|------------------------------------------------------------------------------------------------|
| abs        | numeric_operation | Computes the absolute value                                                                    |
| acos       | numeric_operation | Computes the arccosine of a number                                                             |
| acosh      | numeric_operation | Inverse hyperbolic cosine function                                                             |
| asin       | numeric_operation | Computes the arcsine of a number                                                               |
| asinh      | numeric_operation | Inverse hyperbolic sine function                                                               |
| atan       | numeric_operation | Computes the arctangent of a number                                                            |
| atan2      | numeric_gate      | Computes the four quadrant arctangent in radians                                               |
| atanh      | numeric_operation | Inverse hyperbolic tangent function                                                            |
| cbrt       | numeric_operation | Returns the cube root of a number                                                              |
| ceil       | numeric_operation | Returns the smallest integer greater than or equal to a number                                 |
| cos        | numeric_operation | Computes the cosine of a number (in radians)                                                   |
| cosh       | numeric_operation | Hyperbolic cosine function                                                                     |
| exp        | numeric_operation | Returns e^(input), (the exponential function)                                                  |
| exp2       | numeric_operation | Returns 2^(input)                                                                              |
| floor      | numeric_operation | Returns the largest integer less than or equal to a number                                     |
| fract      | numeric_operation | Returns the fractional part of a number                                                        |
| hypot      | numeric_gate      | Calculates the length of the hypotenuse of a right-angle triangle given legs of length x and y |
| ln         | numeric_operation | Returns the natural logarithm of the number                                                    |
| log        | numeric_gate      | Returns the logarithm of the number with respect to an arbitrary base                          |
| log2       | numeric_operation | Returns the base 2 logarithm of the number                                                     |
| log10      | numeric_operation | Returns the base 10 logarithm of the number                                                    |
| pow        | numeric_gate      | Raises a number to a power                                                                     |
| recip      | numeric_operation | Takes the reciprocal (inverse) of a number, 1/x                                                |
| round      | numeric_operation | Returns the nearest integer to a number. Round half-way cases away from 0.0                    |
| signum     | numeric_operation | Returns a number that represents the sign of the input                                         |
| sin        | numeric_operation | Computes the sine of a number (in radians)                                                     |
| sinh       | numeric_operation | Hyperbolic sine function                                                                       |
| sqrt       | numeric_operation | Returns the square root of a number                                                            |
| tan        | numeric_operation | Computes the tangent of a number (in radians)                                                  |
| tanh       | numeric_operation | Hyperbolic tangent function                                                                    |
| to_degrees | numeric_operation | Converts radians to degrees                                                                    |
| to_radians | numeric_operation | Converts degrees to radians                                                                    |
| trunc      | numeric_operation | Returns the integer part of a number                                                           |

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                               |           |                                                                   |
|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-numeric/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
