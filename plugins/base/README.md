# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module | Functionality                                                        | Tests                                                                                                                                                |
|---------------------|--------|------------|----------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | Base       | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-base">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-base) |

### About this plugin

This plugin provides some essential components, entity types and relation types.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/actions/workflow/status/inexorgame/inexor-rgf-plugin-base/rust.yml">](https://github.com/inexorgame/inexor-rgf-plugin-base/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-plugin-base">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-plugin-base">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-plugin-base">](https://app.codecov.io/gh/inexorgame/inexor-rgf-plugin-base)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-plugin-base">](https://github.com/inexorgame/inexor-rgf-plugin-base/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Components

| Name        | Properties   | DataType | SocketType | Description                                                                                                                             |
|-------------|--------------|----------|------------|-----------------------------------------------------------------------------------------------------------------------------------------|
| named       | name         | string   | None       |                                                                                                                                         |
| describable | description  | string   | None       |                                                                                                                                         |
| flow_2d     | f2dx         | number   | None       |                                                                                                                                         |
|             | f2dy         | number   | None       |                                                                                                                                         |
|             | f2dw         | number   | None       |                                                                                                                                         |
|             | f2dh         | number   | None       |                                                                                                                                         |
| flow_3d     | f3dx         | number   | None       |                                                                                                                                         |
|             | f3dy         | number   | None       |                                                                                                                                         |
|             | f3dz         | number   | None       |                                                                                                                                         |
|             | f3dw         | number   | None       |                                                                                                                                         |
|             | f3dh         | number   | None       |                                                                                                                                         |
|             | f3dd         | number   | None       |                                                                                                                                         |
| licensed    | license      | string   | None       | The SPDX license identifier. See: https://spdx.org/licenses/                                                                            |
|             | attribution  | string   | None       | Title, author, source and license. Best practices for attribution: https://wiki.creativecommons.org/wiki/best_practices_for_attribution |
| versioned   | version      | string   | None       | The version number. Use semantic versioning. See: https://semver.org/                                                                   |

#### Entity Types

| Name    | Properties   | DataType | SocketType | Description                                                                                                                             |
|---------|--------------|----------|------------|-----------------------------------------------------------------------------------------------------------------------------------------|
| comment | name         | string   | None       |                                                                                                                                         |
|         | description  | string   | None       |                                                                                                                                         |

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                            |           |                                                                   |
|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-plugin-base/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
