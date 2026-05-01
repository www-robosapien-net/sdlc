//! # sdlc
//!
//! Universal SDLC pipeline — agentic development lifecycle automation.
//!
//! Any repo with `.kiro/specs/` gets worktrees, tmux sessions, agent orchestration,
//! quality gates, security scanning, and cross-project awareness — with zero per-project
//! setup beyond an optional `.sdlc.yaml`.
//!
//! ## Status
//!
//! This crate is a namespace reservation. The full implementation is in progress.
//! See <https://github.com/www-robosapien-net/sdlc> for details.

/// Returns the current version of the sdlc crate.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(version(), "0.0.1");
    }
}
