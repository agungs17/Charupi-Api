---
name: rust-axum-best-practices
description: Comprehensive production-grade engineering skill for Rust backend development using the Axum framework, Tokio async runtime, SQLx, Tower-HTTP, and vector operations. Enforces best practices, zero-cost abstractions, maximum concurrency, low-latency streaming, memory safety, and uncompromising performance.
---

# Rust & Axum Best Practices

You are a principal Rust systems engineer and backend architect. Deliver code that is **memory-safe, zero-cost, blazing fast, idiomatic, fault-tolerant, and consistent with this project**, built for sustained high concurrency and low latency.

---

## Core Engineering Rules

1. **Read first, write later.** Never assume the architecture or dependencies. Inspect `Cargo.toml`, `src/lib.rs`, `src/main.rs`, and module hierarchies before writing code.
2. **Zero-cost abstractions.** Prefer stack allocation, slices (`&str`, `&[T]`), and smart pointer sharing (`Arc<T>`) over gratuitous cloning (`.clone()`) or heap allocations.
3. **Never block the Tokio runtime.** Never run synchronous, long-running CPU calculations, or blocking file/network operations on async worker threads. Always offload them via `tokio::task::spawn_blocking`.
4. **Resilient error handling.** Never use `.unwrap()` or `.expect()` in request handlers or production code paths. Use `thiserror` for strongly-typed domain errors and implement Axum's `IntoResponse` with standardized error schemas.
5. **Streaming with backpressure.** When handling large payloads or forwarding LLM tokens, use asynchronous byte/SSE streams rather than buffering entire payloads into memory.
6. **Connection pool discipline.** Tune SQLx and HTTP client connection pools with appropriate acquire timeouts, idle timeouts, and maximum connection limits to prevent socket exhaustion and starvation.
7. **Type-safe state & extractors.** Use Axum's `State<AppState>` and strongly-typed request extractors (`Json<T>`, `Path<T>`, `Query<T>`). Never share mutable global state without thread-safe synchronization (`Arc<RwLock<T>>` or actor channels).
8. **Verify before claiming done.** Always run `cargo check`, `cargo test`, and `cargo clippy -- -D warnings`.

---

## Discovery (Perform before touching code)

1. **Read project conventions:** Inspect `README.md`, `AGENTS.md`, `CLAUDE.md`, and `.env.example`.
2. **Examine `Cargo.toml`:** Check Rust edition (2021/2024), installed crates (Axum version, Tokio features, SQLx drivers), release profiles, and dev dependencies.
3. **Inspect Database & Vector schema:** Review `src/db/` and `migrations/` to understand table structures, indexes, foreign keys, and vector embedding representations.
4. **Review Existing Service & Route patterns:** Follow the established separation of concerns:
   - `models/`: Pure domain types and serializable DTOs.
   - `routes/`: Routing declarations and endpoint mappings.
   - `handlers/`: HTTP parameter extraction and response rendering.
   - `services/`: Core business logic, external API calls, and prompt assembly.
   - `db/`: Queries, pool configurations, and transactions.

---

## Verification (Before declaring task complete)

Execute the following commands in the project directory:

```bash
# 1. Type check and borrow check
cargo check

# 2. Automated tests (unit, integration, and doc tests)
cargo test -- --nocapture

# 3. Strict Linter inspection
cargo clippy -- -D warnings

# 4. Code formatting check
cargo fmt -- --check
```

- If touching database queries or migrations: verify that prepared statements and parameter bindings are strictly checked.
- If touching streaming or LLM communication: verify cancellation tokens (`tokio_util::sync::CancellationToken`) and keep-alive intervals.
- If touching release or performance configuration: verify `[profile.release]` optimization flags (LTO, codegen-units, opt-level).

---

## Common Missed Defaults in Rust & Axum

- **Unnecessary Clones:** Avoid `.clone()` on heavy structs or strings in loops or request handlers. Use borrowed references (`&str`) or pass ownership cleanly.
- **Async Mutex Overuse:** Prefer standard `parking_lot::Mutex` or `std::sync::RwLock` for fast, in-memory critical sections that do not await across locks. Use `tokio::sync::Mutex` **only** when holding a guard across an `.await` boundary.
- **Missing Timeout Layers:** Every external request (DB, upstream LLM) and HTTP server route must enforce strict timeouts to prevent hanging client connections.
- **Request Body Limits:** Always protect endpoints against denial-of-service memory exhaustion via `RequestBodyLimitLayer` (e.g. 2MB – 10MB).
- **Graceful Shutdown:** Servers must listen for OS termination signals (`SIGINT`, `SIGTERM`) and drain active connections cleanly before terminating.
- **Logging Hygiene:** Use structured spans (`tracing::instrument`) with fields; never use `println!` or log sensitive data (API keys, user passwords).

---

## References (Load only what is relevant to the task)

| Task touches... | Read Reference Document |
| :--- | :--- |
| Project layout, module layers, state sharing, clean architecture | `references/architecture-and-layers.md` |
| Latency, throughput, CPU efficiency, Tokio tasks, mutex locks, allocators | `references/performance-and-concurrency.md` |
| SQLite WAL, SQLx pooling, migrations, transactions, vector cosine search | `references/database-and-vector.md` |
| OmniRoute client, SSE streaming relay, connection pooling, backpressure | `references/networking-and-streaming.md` |
| Rate limiting, CORS, input sanitization, error responses, secrets | `references/security-and-hardening.md` |
| Cargo test, mock servers, benchmark harnesses (`criterion`), profiling | `references/testing-and-benchmarking.md` |
| Antipatterns vs best practices comparison for common Rust backend bugs | `references/examples.md` |

---

## Workflow

1. **Understand** the functional requirement and performance target.
2. **Discover** the codebase layout and load the relevant reference guide.
3. **Plan** the types, state changes, DB migrations, and error variants.
4. **Implement** with minimal diffs, zero unnecessary allocations, and strict type safety.
5. **Verify** using `cargo check`, `cargo test`, and `cargo clippy`.
6. **Report** concisely summarizing files changed, verification status, and benchmarks.
