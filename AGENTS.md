# Charupi-Api - Agent Instructions

Welcome to Charupi-Api! This document provides instructions, project conventions, and references for all AI coding agents (Antigravity, Gemini CLI, Claude Code, Cursor, Windsurf, Copilot, Aider, Devin, and others).

---

## 1. Project Overview

- **Project Name:** Charupi-Api
- **Role:** High-Performance Intermediary Gateway & Business Logic Service between Charupi-App (React Native) and OmniRoute (LLM Gateway).
- **Framework:** Axum 0.7+
- **Async Runtime:** Tokio 1.43+ (Multi-threaded)
- **Language:** Rust (Edition 2021)
- **Database & Storage:** SQLite via SQLx with WAL mode & vector cosine distance operations.
- **Key Crates:** `axum`, `tokio`, `tower`, `tower-http`, `sqlx`, `reqwest`, `serde`, `serde_json`, `tracing`, `thiserror`.

---

## 2. Essential Commands

Always verify changes before marking a task as complete:

- **Type & Borrow Check:** `cargo check`
- **Unit & Integration Tests:** `cargo test`
- **Linter (Strict Mode):** `cargo clippy -- -D warnings`
- **Format Check:** `cargo fmt -- --check`
- **Run Locally:** `cargo run`
- **Build Release:** `cargo build --release`
- **Docker Compose:** `docker compose up -d`

---

## 3. Specialized Skills

This repository includes a comprehensive, production-grade Rust & Axum engineering skill:

- **Primary Skill Path:** [SKILL.md](file:///.agents/skills/rust-axum-best-practices/SKILL.md) (also available at [SKILL.md](file:///.agent/skills/rust-axum-best-practices/SKILL.md))
- **Skill Name:** `rust-axum-best-practices`
- **References Directory:** `.agents/skills/rust-axum-best-practices/references/`
  - [Architecture & Layers](file:///.agents/skills/rust-axum-best-practices/references/architecture-and-layers.md)
  - [Performance & Concurrency](file:///.agents/skills/rust-axum-best-practices/references/performance-and-concurrency.md)
  - [Database & Vector Operations](file:///.agents/skills/rust-axum-best-practices/references/database-and-vector.md)
  - [Networking & Streaming Relay](file:///.agents/skills/rust-axum-best-practices/references/networking-and-streaming.md)
  - [Security & Hardening](file:///.agents/skills/rust-axum-best-practices/references/security-and-hardening.md)
  - [Testing & Benchmarking](file:///.agents/skills/rust-axum-best-practices/references/testing-and-benchmarking.md)
  - [Antipatterns vs Best Practices](file:///.agents/skills/rust-axum-best-practices/references/examples.md)

Whenever you work on features, bug fixes, refactoring, performance optimization, or native integrations in this repository, **strictly load and follow `rust-axum-best-practices`**.

---

## 4. Key Engineering Rules

1. **Read Before Writing:** Inspect existing patterns in `src/` before creating new files or modifying architecture.
2. **Never Block Tokio:** Offload CPU-heavy or synchronous tasks to `tokio::task::spawn_blocking`.
3. **No Unwraps in Production:** Use `thiserror` and `AppError` returning `AppResult<T>`; never use `.unwrap()` or `.expect()` in request handlers.
4. **Zero-Cost Streaming:** Relay SSE streams token-by-token directly to mobile clients with backpressure support; avoid buffering massive payloads into RAM.
5. **Connection Pool Hygiene:** Always configure SQLx pools with WAL mode and acquire timeouts.
6. **Minimal Diffs:** Make only the requested changes; do not introduce gratuitous refactoring or unasked dependencies.
7. **Verification:** Always run `cargo check`, `cargo test`, and `cargo clippy -- -D warnings` before submitting work.
