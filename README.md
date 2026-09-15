# liznote - Cross-Platform Rust CLI Todo List

A lightweight, fast, and cross-platform todo list application built with Rust. Store your tasks locally with persistent JSON storage.

## Features

✨ **Cross-Platform**: Works on Windows 11, macOS, and Linux
🚀 **Fast**: Built with Rust for blazing-fast performance
💾 **Persistent Storage**: Tasks saved in local JSON files
🎨 **Colored Output**: Beautiful terminal UI with colored text
📱 **CLI-Native**: Simple command-line interface
🔧 **Easy to Use**: Intuitive subcommands for all operations

## Installation

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/)

```bash
# Clone the repository
git clone https://github.com/supercubegame/liznote.git
cd liznote

# Build and install
cargo install --path .
```

## Usage

### Add a task
```bash
liznote add "Buy groceries"
liznote add "Fix bug in production"
```

### List all tasks
```bash
liznote list
liznote list --pending  # Show only pending tasks
liznote list --done     # Show only completed tasks
```

### Mark task as done
```bash
liznote done 1
```

### Remove a task
```bash
liznote remove 1
```

### Clear all tasks
```bash
liznote clear
```

## Data Storage

Tasks are stored in a JSON file located at:
- **Windows**: `%APPDATA%\liznote\todos.json`
- **macOS/Linux**: `~/.local/share/liznote/todos.json`

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run release tests (optimized)
cargo test --release
```

## CI/CD

This project uses GitHub Actions for:
- Testing on Ubuntu, Windows, and macOS (Rust stable & beta)
- Clippy linting checks
- Code formatting validation
- Cross-platform binary builds
- Code coverage reporting

## Project Structure

```
liznote/
├── src/
│   └── main.rs          # Main application code
├── .github/
│   └── workflows/
│       └── ci.yml       # GitHub Actions CI/CD configuration
├── Cargo.toml           # Project manifest
└── README.md            # This file
```

## Dependencies

- **serde**: Serialization/deserialization framework
- **serde_json**: JSON support
- **clap**: Command-line argument parsing
- **chrono**: Date and time handling
- **colored**: Terminal color support
- **dirs**: Cross-platform directory paths

## Building for Release

```bash
cargo build --release

# Binary location: target/release/liznote
# For Windows: target/release/liznote.exe
```

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Feel free to:
1. Fork the repository
2. Create a feature branch
3. Submit a pull request

## Author

Randy Hopkins

---

**Status**: 🚀 Ready for use | ✅ Tests passing | 📦 Cross-platform support