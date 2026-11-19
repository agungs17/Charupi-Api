# Security & Hardening in Axum

Backend services must be resilient against denial-of-service, malicious inputs, unauthorized access, and resource exhaustion.

---

## 1. Request Body Size Limiting

Without request body limits, an attacker can send an unbounded stream of bytes and exhaust server memory (OOM crash).

### Global Middleware Limit:
```rust
use tower_http::limit::RequestBodyLimitLayer;

let app = Router::new()
    // ...
    .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024)); // 10MB limit
```

For endpoints that accept smaller payloads, specify route-level extractors or use lower thresholds.

---

## 2. Cross-Origin Resource Sharing (CORS)

Configure CORS explicitly to avoid unauthorized browser-based requests while permitting mobile app requests:

```rust
use tower_http::cors::{Any, CorsLayer};

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}
```

---

## 3. SQL Injection Defense

Never concatenate strings into SQL queries. Always use SQLx parameterized queries with `.bind()`:

```rust
// BAD: Vulnerable to SQL injection!
let query = format!("SELECT * FROM messages WHERE content = '{}'", user_input);
sqlx::query(&query).fetch_all(&pool).await?;

// GOOD: Fully parameterized and safely escaped by SQLite engine
sqlx::query("SELECT * FROM messages WHERE content = ?")
    .bind(user_input)
    .fetch_all(&pool)
    .await?;
```

---

## 4. Upstream Secrets & Environment Hygiene

- Never log API keys or Authorization bearer tokens in traces or logs.
- Redact secrets using custom `Debug` implementations or the `secrecy` crate:

```rust
#[derive(Clone)]
pub struct SecretApiKey(String);

impl std::fmt::Debug for SecretApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SecretApiKey([REDACTED])")
    }
}
```

---

## 5. Standardized Error Masking

In production mode, internal database error messages or system paths should never be leaked to clients.
Map internal errors to generic messages in `into_response()` while logging the detailed root cause internally:

```rust
AppError::Database(err) => {
    tracing::error!(error = ?err, "Database error occurred");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "DATABASE_ERROR",
        "A database error occurred".to_string(), // Safe client message
    )
}
```
