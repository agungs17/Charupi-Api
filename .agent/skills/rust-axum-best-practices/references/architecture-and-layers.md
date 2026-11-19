# Architecture & Modular Layering in Axum

A clean, maintainable, and high-performance Rust backend segregates responsibilities across isolated layers:

```
┌─────────────────────────────────────────────────────────────┐
│                       HTTP Network                          │
│     (Client Requests: REST / SSE Stream / WebSockets)       │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                     Tower Middleware                        │
│   (TraceLayer, CorsLayer, CompressionLayer, TimeoutLayer)   │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                      Routing Layer                          │
│    (src/routes/ — Route declaration, URL parameter binding) │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                     Handler Layer                           │
│   (src/handlers/ — Request extractors, status code mapping) │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                     Service Layer                           │
│ (src/services/ — Domain logic, prompt assembly, LLM client) │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                  Data / Repository Layer                    │
│      (src/db/ — SQLx connection pool, queries, vectors)     │
└─────────────────────────────────────────────────────────────┘
```

---

## 1. Directory Structure

```
src/
├── main.rs         # Application entry point, signal handling, server run
├── lib.rs          # Module declarations and library exports
├── config.rs       # Strongly-typed environment variables (from .env)
├── error.rs        # AppError enum, Result<T>, and IntoResponse
├── state.rs        # Shared AppState struct (Pool, HTTP Client, Config)
├── routes/         # Route definitions grouped by domain
├── handlers/       # Request handlers (extractors -> service call -> response)
├── services/       # Pure business logic and upstream integrations
├── models/         # Domain entities, DTOs, and serialization schemas
├── db/             # Pool setup, migrations, and raw query operations
└── middleware/     # Reusable Tower service middleware layers
```

---

## 2. State Management Guidelines

In Axum 0.7+, application state is passed using `axum::extract::State`.

### Rules for `AppState`:
- **Must be cheaply cloneable**: Wrap large or non-cloneable objects in `std::sync::Arc`.
- **Prefer immutable shared state**: If read-heavy, use `Arc<T>`.
- **Granular synchronization**: If mutable state is required, protect only the specific sub-field with `Arc<RwLock<T>>` or `tokio::sync::mpsc::Sender<Command>`, never lock the entire `AppState`.

```rust
// GOOD: Cheaply cloneable AppState
#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,          // SqlitePool is internally Arc-wrapped
    pub http_client: reqwest::Client,  // Client is internally Arc-wrapped
    pub config: Arc<AppConfig>,        // Explicit Arc for config
}
```

---

## 3. Thin Handlers, Rich Services

- **Handlers** should only:
  1. Extract parameters (`Path`, `Query`, `Json`, `State`).
  2. Validate input constraints.
  3. Call domain service functions.
  4. Return an idiomatic response (`Json<T>`, `Sse<Stream>`, `StatusCode`).
- **Services** should contain all business logic, prompt assembly, upstream API orchestrations, and database transactions. Handlers should not contain raw SQL queries or business algorithms.

---

## 4. Error Propagation Flow

All handlers and service functions return `AppResult<T>`. When an error occurs:
- Use the `?` operator for clean propagation.
- Convert external errors into domain errors automatically via `#[from]` attributes in `thiserror`.
- Never catch and discard errors silently without structured logging.
