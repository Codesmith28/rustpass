# RustPass: A Secure Password Manager

RustPass is a simple, secure password manager written in Rust that provides both a Terminal User Interface (TUI) and command-line interface. It offers strong encryption, easy password management, and a focus on security.

## Features

- **Encrypted Storage**: All passwords are encrypted using AES-256-GCM with Argon2 key derivation
- **Terminal User Interface**: Full-featured TUI for managing passwords
- **Command-Line Interface**: Fast access through CLI commands
- **Multi-Select**: Ability to perform operations on multiple entries
- **Search Capabilities**: Fuzzy search to quickly find passwords
- **Clipboard Integration**: Copy passwords to clipboard with a keystroke
- **Secure by Default**: Password file permissions limited to owner read/write only

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/username/rustpass.git
cd rustpass

# Build the project
cargo build --release

# Run the binary
./target/release/rsp
```

### Using Release Script

```bash
# Run the release script to build for current architecture
./release.sh

# The binary will be copied to current directory
./rsp
```

## Usage

### TUI Mode

```bash
# Start the Terminal UI
rsp tui
```

#### TUI Keybindings

- **↑/↓**: Navigate list
- **Tab**: Multi-select current & move to next
- **Alt+c**: Copy password to clipboard
- **Alt+e**: Edit selected entry
- **Alt+d**: Delete entry (or delete multi-selected)
- **Alt+n**: Create new entry
- **Alt+h**: Toggle help panel
- **Esc/q**: Quit

### CLI Commands

```bash
# Add a new password
rsp add <name> <username> <password>

# List all passwords
rsp list

# Remove a password
rsp remove <name>

# Unlock the password manager
rsp unlock [password]

# Lock the password manager
rsp lock

# Show help
rsp help
```

## Security

RustPass employs several security measures:

- Strong encryption with AES-256-GCM
- Argon2id for key derivation
- Password file permissions restricted to 600 (owner read/write only)
- No passwords are stored in plaintext
- Multiple encryption layers for sensitive data

## Development

### Requirements

- Rust 1.65+
- Linux/Unix system (for proper file permissions)

### Building

```bash
# Development build
cargo build

# Run tests
cargo test

# Release build
cargo build --release
```

## License

MIT License

## Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) for the terminal interface
- Uses [Argon2](https://github.com/password-hashing/argon2) for secure password hashing
- Uses [AES-GCM](https://github.com/RustCrypto/AEADs) for encryption
