# hlavi-tui

Interactive TUI (Terminal User Interface) for Hlavi kanban task management with AI agent support.

## Table of Contents

- [Getting Started](#getting-started)
- [Documentation](#documentation)
- [Development](#development)
- [Contributing](#contributing)
- [Contact](#contact)

## Getting Started

A quick guide on how you can get started running and working on the applicatoin on your local machine.

### Requirements

- Rust 1.75 or higher
- Cargo
- Terminal with UTF-8 and color support

### Clone

```bash
git clone https://github.com/mmuhlariholdings/hlavi-tui.git
cd hlavi-tui
```

### Install

```bash
cargo install --path .
```

### Usage

Launch the TUI from any directory with an initialized Hlavi board:

```bash
hlavi-tui
```

### Keybindings

| Key | Action |
|-----|--------|
| `h` or `←` | Move to previous column |
| `l` or `→` | Move to next column |
| `j` or `↓` | Move down in current column |
| `k` or `↑` | Move up in current column |
| `r` | Reload board |
| `q` | Quit |

## Documentation

The TUI provides a visual, column-based interface for managing your Hlavi tasks. It displays tickets organized by status (TODO, In Progress, Review, Done) in a kanban board layout.

Features:
- **Visual Kanban Board**: See all your tickets organized in columns
- **Keyboard-Driven**: Fast navigation with vi-style keybindings
- **Acceptance Criteria Tracking**: Visual indicators showing completion progress
- **Real-time Reload**: Refresh your board instantly

## Development

During development, use `cargo run` instead of installing the TUI every time. This is much faster and allows for quick iteration.

```bash
# Run the TUI directly (no installation needed)
cargo run
```

### Testing

Run tests to validate your changes:

```bash
# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

### When to Install

Only install when you need to:

1. **Test the final user experience:**
   ```bash
   cargo install --path .
   hlavi-tui
   ```

2. **Validate before release** - Ensure the installed version works correctly

3. **Use it for real work** - When actually using hlavi to manage tasks

## Contributing

Take a moment to review our [contribution guide](CONTRIBUTING.md) before submitting your first pull request.

Make sure that you check for open issues and pull requests to see if someone else is working on something similar.

## Contact

For feedback, requests or enquiries:

🌐 [http://www.mmuhlariholdings.co.za](http://www.mmuhlariholdings.co.za)
