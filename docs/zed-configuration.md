I use Zed as my main editor. To make my Java development faster and easier, I wrote bloomery and integrated it directly into Zed's tasks and keybindings.

This way, I can run or build Java projects or single files using Zed without going into the Terminal.

### Zed Tasks

`tasks.json:`

```json
[
  {
    "label": "BLM Run",
    "command": "blm run",
    "cwd": "$ZED_WORKTREE_ROOT",
    "allow_concurrent_runs": false
  },
  {
    "label": "BLM Run File",
    "command": "blm run-file $ZED_FILE",
    "cwd": "$ZED_WORKTREE_ROOT",
    "allow_concurrent_runs": false
  },
  {
    "label": "BLM Build",
    "command": "blm build",
    "cwd": "$ZED_WORKTREE_ROOT"
  },
  {
    "label": "BLM Build File",
    "command": "blm build-file $ZED_FILE",
    "cwd": "$ZED_WORKTREE_ROOT"
  },
  {
    "label": "BLM Clean",
    "command": "blm clean",
    "cwd": "$ZED_WORKTREE_ROOT"
  }
]
```

### Keybindings

I like to use the combination between the CTRL key and the number keys.

| Shortcut | Action                  |
| -------- | ----------------------- |
| `Ctrl+1` | Run Java project        |
| `Ctrl+2` | Run current Java file   |
| `Ctrl+3` | Build Java project      |
| `Ctrl+4` | Build current Java file |

`keymap.json:`

```json
{
  "context": "Editor && extension == java",
  "bindings": {
    "ctrl-1": [
      "task::Spawn",
      {
        "task_name": "BLM Run"
      }
    ],
    "ctrl-2": [
      "task::Spawn",
      {
        "task_name": "BLM Run File"
      }
    ],
    "ctrl-3": [
      "task::Spawn",
      {
        "task_name": "BLM Build"
      }
    ],
    "ctrl-4": [
      "task::Spawn",
      {
        "task_name": "BLM Build File"
      }
    ]
  }
}
```

This keeps my Bloomery workflow simple: **write code → press a shortcut → see the result → continue.**
