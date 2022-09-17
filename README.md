# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module | Functionality                                                        | Tests                                                                                                                                                          |
|---------------------|--------|------------|----------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | Connector  | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-connector">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-connector) |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-connector/main/docs/images/inexor_2.png">
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

This plugin provides connectors.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/inexorgame/inexor-rgf-plugin-connector/Rust">](https://github.com/inexorgame/inexor-rgf-plugin-connector/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-plugin-connector">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-plugin-connector">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-connector">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-connector)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-plugin-connector">](https://github.com/inexorgame/inexor-rgf-plugin-connector/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### User Stories

* As a developer or a flow editor I can connect and disconnect an inbound property of an entity instance with an
  outbound property of another entity instance
* Changes on the inbound property results on changes of the outbound property

#### What is a connector?

A connector connects a property of the outbound entity instance with a property of the inbound entity
instance and propagates the changes of the value.

During propagation a propagation function is called. The propagation function has one single input (the
incoming value). Connectors of different types has different propagation functions.

The propagation function can only do simple things (like casting or logging) but in fact even these
simple operations makes the control flow much simpler and much more readable.

#### How does a connector work?

The connector is a relation instance which connects two entity instances. The relation itself stores
the names of the output property and the input property.

In theory, it's also possible to connect two properties of the same entity instance.

On construction the streams are connected.

On destruction of the connector, the stream will be removed.

---
**Warning**

1. Connecting properties of the same entity instance is discouraged to prevent feedback loops
2. No type checks are performed on construction (yet; you are responsible)
3. There is no check about feedback loops (yet; you are responsible)
4. Renaming the relation properties (outbound_property_name, inbound_property_name) doesn't have any
   effect (yet). You have to remove the old connector and create a new connector.

---

#### Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

#### Components

| Name                | Description                                                                             | Properties             | Data Type | Socket Type |
|---------------------|-----------------------------------------------------------------------------------------|------------------------|-----------|-------------|
| connector           | Connects two properties                                                                 | outbound_property_name | string    | none        |
|                     |                                                                                         | inbound_property_name  | string    | none        |
| buffer              | A buffer for FIFOs and interpolation                                                    | buffer_size            | number    | none        |
|                     |                                                                                         | buffer                 | array     | none        |
| propagation_counter | Counts connector propagations. This component can be applied on all types of connectors | propagation_count      | number    | none        |

#### Relation Types

| Name                            | Components | Description                                                                                        |
|---------------------------------|------------|----------------------------------------------------------------------------------------------------|
| buffered_fifo_connector         | connector  | This connector propagates the first inserted value of the FIFO buffer with the given size          |
|                                 | buffer     |
| debounce_connector              | connector  | This connector propagates the value if and only if the value is different                          |
| debug_connector                 | connector  | This connector logs the value before propagation (log level debug)                                 |
| default_connector               | connector  | This is the default connector type, which simply does nothing than propagate the value             |
| delay_connector                 | connector  | This connector propagates the value after a given duration. This operation is blocking             |
| numeric_interpolation_connector | connector  | This connector propagates the average of the numeric elements in the buffer                        |
|                                 | buffer     |
| parse_float_connector           | connector  | This connector parses a string value and propagates a float value                                  |
| parse_int_connector             | connector  | This connector parses a string value and propagates a int value                                    |
| to_string_connector             | connector  | This connector converts the value of any type to string before propagation                         |
| trace_connector                 | connector  | This connector logs the value before propagation (log level trace)                                 |
| increment_by_connector          | connector  | This connector adds the value of the outbound property to the value of the inbound property        |
| decrement_by_connector          | connector  | This connector subtracts the value of the outbound property from the value of the inbound property |

##### Future: More (useful) connectors

| Name                | Components | Description                                                              |
|---------------------|------------|--------------------------------------------------------------------------|
| str_split_connector | connector  | A string is split into tokens. Propagates an JSON array of string tokens |
| str_join_connector  | connector  | Joins an array of strings and propagates the resulting string            |

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                                 |           |                                                                   |
|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-connector/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
