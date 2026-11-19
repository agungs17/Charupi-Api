# Charupi-Api - Guidelines for AI Agents

Follow the instructions and conventions in this repository:

- **Core Agent Guidelines:** See [AGENTS.md](./AGENTS.md)
- **Primary Skill:** Read [.agents/skills/rust-axum-best-practices/SKILL.md](./.agents/skills/rust-axum-best-practices/SKILL.md) and its reference documents before building or modifying code.

## Commands
- Type Check: `cargo check`
- Test: `cargo test`
- Lint: `cargo clippy -- -D warnings`
- Run: `cargo run`

## Rules
- Deliver minimal diffs.
- Always run `cargo check` and `cargo test` to verify changes.
- Never use `.unwrap()` or `.expect()` in request handlers.
- Strictly adhere to `rust-axum-best-practices`.
