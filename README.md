# Rust Text Editor

A modern, terminal-based text editor written in Rust, inspired by nano. This editor provides a simple yet powerful interface for editing text files directly in your terminal.

## Features

- 🚀 Fast and efficient text editing
- 📝 Support for basic text operations (insert, delete, newline)
- 🎯 Intuitive cursor movement
- 💾 File saving and loading
- 📊 Status bar with file information
- 🎨 Clean and modern terminal interface
- 🔄 Real-time file modification tracking
- ⌨️ Support for common keyboard shortcuts

## Installation

### Prerequisites

- Rust and Cargo (latest stable version)
- A terminal that supports ANSI escape sequences

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/RKSM-GIT/rust-text-editor.git
cd rust-text-editor
```

2. Build the project:
```bash
cargo build --release
```

3. The executable will be available at `target/release/rust-text-editor`

## Usage

### Starting the Editor

You can start the editor in two ways:

1. Start with an empty buffer:
```bash
cargo run
```

2. Open an existing file:
```bash
cargo run path/to/your/file.txt
```

For example, to open a file named `poem.txt`:
```bash
cargo run poem.txt
```

### Basic Commands

- `Ctrl + S`: Save the current file
- `Ctrl + Q`: Quit the editor (press multiple times if there are unsaved changes)
- Arrow keys: Move the cursor
- `Home/End`: Move to start/end of line
- `Page Up/Page Down`: Scroll through the document
- `Backspace`: Delete character before cursor
- `Delete`: Delete character after cursor
- `Enter`: Insert new line

### Opening Files

To open a file:
```bash
rust-text-editor path/to/your/file.txt
```

If no file is specified, the editor will start with an empty buffer.

## Development

### Project Structure

```
src/
├── editor/
│   ├── command/     # Command handling
│   ├── view/        # View and buffer management
│   ├── terminal.rs  # Terminal interface
│   └── ...
└── main.rs         # Entry point
```

### Building for Development

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the nano text editor
- Built with [crossterm](https://github.com/crossterm-rs/crossterm) for terminal manipulation
- Uses [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation) for proper text handling

## Roadmap

- [ ] Syntax highlighting
- [ ] Searching
- [ ] Search and replace functionality

## Support

If you encounter any issues or have questions, please open an issue in the GitHub repository.
