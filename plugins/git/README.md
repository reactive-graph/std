# Plugin Git

This plugin provides git functionality. You can clone a repository, fetch, pull and fast-forward.

## Component

| Property        | Component           | DataType  | SocketType | Description                                               |
|-----------------|---------------------|-----------|------------|-----------------------------------------------------------|
| `name`          | `base__named`       | `String`  | `none`     | The name.                                                 |
| `description`   | `base__describable` | `String`  | `none`     | The description.                                          |
| `url`           | `http__url`         | `String`  | `input`    | The remote url.                                           |
| `file`          | `file_file`         | `String`  | `input`    | The local path.                                           |
| `branch`        | `git__git`          | `String`  | `input`    | The git branch name. Executes a `git checkout` on change. |
||
| `remote_name`   | `git__git`          | `String`  | `input`    | The name of the remote (default: `origin`).               |
| `remote_branch` | `git__git`          | `String`  | `input`    | The name of the remote branch                             |
||
| `trigger`       | `git__git`          | `Bool`    | `input`    | Clone or pull                                             |
| `fetch`         | `git__git`          | `Bool`    | `input`    | Fetch                                                     |
| `fast_forward`  | `git__git`          | `Bool`    | `input`    | Fast-forward                                              |
| `push`          | `git__git`          | `Bool`    | `input`    | Pushes to the given remote (by default origin)            |
| `merge_from`    | `git__git`          | `String`  | `input`    | Merges the given branch into the current branch           |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

## Credits

This plugin integrates with https://crates.io/crates/git2
