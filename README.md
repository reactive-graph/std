<div align="center">
  <a href="https://www,reactive-graph.io/"><img src="https://raw.githubusercontent.com/reactive-graph/design/main/public/logo/rendered/malachite/reactive-graph-400x400.png" alt="Reactive Graph"></a>
</div>

<h2 align="center">
    <a href="https://std.reactive-graph.io/">std.reactive-graph.io</a>
</h2>

<p align="center">
This repository contains the standard library for the <a href="https://github.com/reactive-graph/reactive-graph">Reactive Graph</a>.
</p>

<p align="center">
  <a href="https://github.com/reactive-graph/reactive-graph">Reactive Graph</a> is a <b>reactive runtime</b> based on a <b>graph database</b>, empowering everyone to build reliable and efficient software.
</p>

<hr>

<div align="center" style="text-align: center">

[<img src="https://img.shields.io/badge/book-master-yellow">](https://docs.reactive-graph.io/book/)
[<img src="https://img.shields.io/badge/api-master-yellow">](https://docs.reactive-graph.io/docs/)

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/license/reactive-graph/std">](https://github.com/reactive-graph/std/blob/main/LICENSE)

[![Build](https://github.com/reactive-graph/std/actions/workflows/rust.yml/badge.svg)](https://github.com/reactive-graph/std/actions/workflows/rust.yml)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

</div>


<h2 align="center" style="text-align: center;">List of Plugins</h2>

In this repository you'll find the essential plugins which are necessary or useful for all use cases.

| Name                                         | Description                                |
|----------------------------------------------|--------------------------------------------|
| [Arithmetic](./plugins/arithmetic/README.md) | Provides arithmetic gates and operations   |
| [Base](./plugins/base/README.md)             | Provides basic components and entity types |
| [Color](./plugins/color/README.md)           | Colors and colorspace transformations      |
| [Comparison](./plugins/comparison/README.md) | Provides comparison gates                  |
| [Config](./plugins/config/README.md)         | Load configuration files                   |
| [Connector](./plugins/connector/README.md)   | Provides property connectors               |
| [Date Time](./plugins/date-time/README.md)   | Date and Time, Durations and calculation   |
| [JSON](./plugins/json/README.md)             | Handles JSON arrays and objects            |
| [Logical](./plugins/logical/README.md)       | Provides logical operations                |
| [Numeric](./plugins/numeric/README.md)       | Numeric operations                         |
| [Random](./plugins/random/README.md)         | Generate random numbers                    |
| [Result](./plugins/result/README.md)         | Result components                          |
| [State](./plugins/state/README.md)           | Debounced states                           |
| [String](./plugins/string/README.md)         | Provides string operations                 |
| [Trigger](./plugins/trigger/README.md)       | Triggers and actions                       |
| [Value](./plugins/value/README.md)           | Values and state management                |

<h2 align="center" style="text-align: center;">Moving plugins to a different repository</h2>

| Repository | Name                                      | Description                    |
|------------|-------------------------------------------|--------------------------------|
| net        | [Git](./plugins/git/README.md)            | Git VCS operations             |
| net        | [HTTP](./plugins/http/README.md)          | HTTP and JSONRPC               |
| meta       | [Meta Data](./plugins/metadata/README.md) | Meta Data - Dublin Core, EXIF  |
| meta       | [Taxonomy](./plugins/taxonomy/README.md)  | Taxonomy - categories and tags |

<h2 align="center" style="text-align: center;">Local Build + Local Deployment</h2>

#### Setup deployment directory in `.deployment.toml`

```shell
target_dirs = [
  "../reactive-graph/plugins/deploy"
]
```

#### Install a specific plugin

```shell
cargo build
cargo post build --package=reactive-graph-plugin-date-time
```

#### Install all plugins of this repository at once

```shell
cargo build
cargo post build --package=deployment-all
```
