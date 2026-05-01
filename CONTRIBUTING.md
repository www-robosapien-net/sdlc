# Contributing to sdlc

Thank you for your interest in contributing to sdlc!

## Development

### Rust
```bash
cd rust
cargo test
cargo clippy
cargo fmt
```

### Go
```bash
cd go
go test ./...
go vet ./...
```

## Pull Requests

- All PRs must target `main`
- CI must pass (tests, clippy, fmt, coverage ≥100%)
- One approval required

## Reporting Issues

Open an issue on GitHub with:
- What you expected
- What happened
- Steps to reproduce
