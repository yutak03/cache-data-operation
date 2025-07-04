# CDOT - Cache Data Operation Tool

A simple interactive CLI tool for managing Redis/Valkey cache data through a menu-based interface.

## Features

- ✅ Interactive CLI menu
- ✅ Key search with pattern matching
- ✅ Get/Set key-value operations
- ✅ Key deletion
- ✅ TTL (Time To Live) support
- ✅ Redis and Valkey compatible

## Requirements

- Rust 1.70 or higher
- Redis or Valkey server

## Installation

### From source

```bash
# Clone the repository
git clone <repository-url>
cd cache-data-operation

# Install using cargo
cargo install --path .
```

### From crates.io (when published)

```bash
cargo install cdot
```

## Usage

### Running the Tool

```bash
# Run the application
cargo run

# Or install and run globally
cargo install --path .
cdot
```

The tool starts in interactive mode and connects to Redis at `redis://127.0.0.1/` by default.

## Interactive Mode

When running in interactive mode, you'll see a menu like this:

```
Cache Data Operation Tool
=========================
Connecting to redis://127.0.0.1/...
Connected successfully!

--- Menu ---
1. Search keys
2. Get value by key
3. Set key-value
4. Delete key
5. Exit
Select option: 
```

### Menu Options

1. **Search keys**: Find keys matching a pattern
2. **Get value by key**: Retrieve the value for a specific key
3. **Set key-value**: Create or update a key-value pair with optional TTL
4. **Delete key**: Remove a key from the cache
5. **Exit**: Quit the application

## Configuration

### Redis/Valkey URL Format

The tool supports standard Redis URL formats:

- `redis://127.0.0.1/` - Default local Redis
- `redis://127.0.0.1:6379/` - Explicit port
- `redis://127.0.0.1:6379/1` - Specific database
- `redis://:password@127.0.0.1:6379/` - With authentication
- `redis://user:password@127.0.0.1:6379/` - With username and password

### Pattern Matching

The search functionality uses Redis KEYS command with patterns:
- `*` - Match any number of characters
- `?` - Match single character
- `[abc]` - Match any character in brackets
- `[^abc]` - Match any character not in brackets

Examples:
- `user:*` - All keys starting with "user:"
- `*:config` - All keys ending with ":config"
- `user:?` - Keys like "user:1", "user:a", etc.

## Technical Details

### Dependencies

- `redis`: Redis/Valkey client library
- `tokio`: Async runtime (for Redis client)
- `anyhow`: Error handling
- `clap`: Command-line argument parsing

### Data Types

The tool works with Redis string data types and can handle:
- Plain text strings
- JSON strings
- Serialized data as strings
- Binary data (displayed as UTF-8 if possible)

### Error Handling

The tool provides clear error messages for common scenarios:
- Connection failures
- Invalid Redis URLs
- Key not found
- Permission denied
- Network timeouts

## Development

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Check without building
cargo check
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test
```

### Project Structure

```
src/
└── main.rs          # Complete application in single file
```

## License

MIT License

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Troubleshooting

### Common Issues

**Connection refused**
- Make sure Redis/Valkey server is running
- Check the URL format and port
- Verify firewall settings

**Authentication failed**
- Ensure correct username/password in URL
- Check Redis AUTH configuration

**Command not found after installation**
- Make sure `~/.cargo/bin` is in your PATH
- Try `source ~/.bashrc` or restart terminal

**Permission denied**
- Check Redis ACL settings
- Verify user permissions for the database