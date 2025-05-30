# Multi-crate Rust Project

This is a multi-crate Rust project that demonstrates a well-organized workspace structure.

## Project Structure

```
.
├── Cargo.toml (workspace configuration)
├── crates/
│   ├── common/ (shared library)
│   ├── core/ (core business logic)
│   └── utils/ (utility functions)
└── bin/
    └── app/ (main application)
```

## Crates

- `common`: Shared types, error definitions, and common utilities
- `core`: Core business logic and services
- `utils`: Helper functions and utilities
- `app`: Main application binary

## Building

```bash
cargo build
```

## Running

```bash
cargo run --bin app
```

## Development

This project uses:
- Rust 2021 edition
- Tokio for async runtime
- Serde for serialization
- Logging with env_logger
- Error handling with thiserror and anyhow