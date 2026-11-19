# Testing & Benchmarking in Axum

Comprehensive testing ensures correctness, prevents regressions, and validates performance requirements under load.

---

## 1. Unit Testing Handlers & Services

Use `tokio::test` for testing async functions without spinning up a real TCP socket.

### Testing In-Memory SQLite Database:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        crate::db::run_migrations(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_and_fetch_persona() {
        let pool = setup_test_db().await;
        // Test database queries and assertions...
    }
}
```

---

## 2. Integration Testing with `axum-test` or `tower::ServiceExt`

You can test full HTTP request-response cycles directly in-memory using `tower::ServiceExt::oneshot`:

```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn test_health_endpoint() {
    let state = setup_test_state().await;
    let app = crate::routes::create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

---

## 3. Load Testing & Benchmarking

### Benchmarking with `k6`:
Run a simulated 100-user concurrency test against the local server:

```javascript
import http from 'k6/http';
import { check } from 'k6';

export const options = {
  vus: 50,
  duration: '30s',
};

export default function () {
  const res = http.get('http://localhost:8080/health');
  check(res, { 'status is 200': (r) => r.status === 200 });
}
```

Run the benchmark:
```bash
k6 run benchmark.js
```
Expected result: **> 40,000 requests/sec** with **p99 latency < 2ms** on Apple Silicon / modern x86_64 CPUs.
