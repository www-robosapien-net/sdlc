# Development Guide

## Repository Structure

```
sdlc/
├── rust/                     # Rust crate (crates.io: sdlc)
│   ├── Cargo.toml
│   ├── deny.toml             # License & advisory policy
│   ├── src/
│   │   ├── lib.rs            # Library
│   │   └── main.rs           # Binary
│   └── .gitignore
├── go/                       # Go module
│   ├── go.mod
│   ├── sdlc.go               # Package
│   └── cmd/sdlc/main.go      # Binary
├── .github/workflows/
│   ├── ci.yml                # Test, lint, coverage, security, cross-compile
│   └── release.yml           # Build binaries on tag push
├── LICENSE                   # MIT
├── CONTRIBUTING.md
└── README.md
```

## Prerequisites

- Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Go: https://go.dev/dl/

## Development

### Rust

```bash
cd rust
cargo test              # Run tests
cargo clippy            # Lint
cargo fmt               # Format
cargo build --release   # Build release binary
cargo run               # Run
```

### Go

```bash
cd go
go test ./...           # Run tests
go vet ./...            # Lint
go build ./cmd/sdlc     # Build binary
go run ./cmd/sdlc       # Run
```

## CI Gates (all must pass)

| Gate | Rust | Go |
|------|------|----|
| Tests | `cargo test` | `go test ./...` |
| Lint | `cargo clippy -- -D warnings` | `go vet ./...` |
| Format | `cargo fmt --check` | — |
| Coverage | `cargo tarpaulin --fail-under 100` | — |
| Security | `cargo audit` + `cargo deny check` | — |
| Cross-compile | 5 targets (linux/mac/win × x86/arm) | 3 OS |

## Publishing

### crates.io

```bash
# Retrieve token from vault
CRATES_TOKEN=$(vault kv get -field=token secret/crates-io/sdlc-publish)

cd rust
cargo login $CRATES_TOKEN
cargo publish --dry-run    # verify first
cargo publish
```

### GitHub Release

```bash
# Tag triggers GitHub Actions release workflow
# Builds 5 platform binaries and attaches to release
git tag v0.1.0
git push origin v0.1.0
```

Platform binaries built automatically:
- `sdlc-linux-x86_64`
- `sdlc-linux-aarch64`
- `sdlc-darwin-x86_64`
- `sdlc-darwin-aarch64`
- `sdlc-windows-x86_64.exe`

### Go Module

```bash
# Tag the go subdirectory
git tag go/v0.1.0
git push origin go/v0.1.0
```

## Branching

| Branch | Purpose |
|--------|---------|
| `main` | Shared: README, CI, LICENSE, schema, test fixtures |
| `feat/100-rust` | Rust implementation (worktree) |
| `feat/100-go` | Go implementation (worktree) |

See spec 100 for the full DAG model.

## Security

- `cargo audit` — CVE scanning on every PR
- `cargo deny` — license compliance (MIT, Apache-2.0, BSD, ISC allowed)
- Dependabot enabled for automated dependency updates
- No secrets in code — tokens stored in Vault

## Vault Secrets

| Path | Contents |
|------|----------|
| `secret/crates-io/sdlc-publish` | crates.io publish token |
| `secret/github/sdlc-ci` | GitHub PAT (repo + workflow scope) |
