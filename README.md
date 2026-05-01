# sdlc

> *Opinions included. Batteries not required.*

Universal SDLC pipeline — agentic development lifecycle automation.

Any repo with `.kiro/specs/` gets worktrees, tmux sessions, agent orchestration, quality gates, security scanning, and cross-project awareness — with zero per-project setup.

## Install

### Rust (crates.io)
```bash
cargo install sdlc
```

### Go
```bash
go install github.com/www-robosapien-net/sdlc/go/cmd/sdlc@latest
```

### From source
```bash
git clone https://github.com/www-robosapien-net/sdlc.git
cd sdlc/rust && cargo build --release
```

## What it does

> *Your agents have opinions. This pipeline ships them.*

- **Spec-driven** — write requirements + tasks, agents do the rest
- **Multi-machine** — federated scrum masters coordinate across hosts
- **Quality gates** — lint, test, coverage, security, architecture review
- **Self-healing** — stall detection, auto-restart, merge recovery
- **Cross-platform** — Linux, macOS, Windows (WSL2), ARM64

## Status

> *Because your backlog won't clear itself.*

Namespace reservation — full native implementation in progress. The current pipeline system runs as bash + Python. This crate will be the v2.0 single-binary replacement.

See the internal project repository for the working pipeline system.

## License

MIT OR Apache-2.0
