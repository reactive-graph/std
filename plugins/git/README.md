# Plugin Git

## Component

| Property      | Component           | DataType | SocketType | Description                                               |
|---------------|---------------------|----------|------------|-----------------------------------------------------------|
| `name`        | `base__named`       | `String` | `none`     | The name.                                                 |
| `description` | `base__describable` | `String` | `none`     | The description.                                          |
| `url`         | `http__url`         | `String` | `none`     | The remote url.                                           |
| `file`        | `file_file`         | `String` | `none`     | The local path.                                           |
| `branch`      | `git__git`          | `String` | `none`     | The git branch name. Executes a `git checkout` on change. |
||
| `merge_from`  | `git__git`          | `String` | `input`    | Merges the given branch into the current branch           |
| `push`        | `git__git`          | `String` | `input`    | Pushes to the given remote (by default origin)            |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

## Credits

This plugin integrates with https://crates.io/crates/git2
