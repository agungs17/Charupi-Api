# Antipatterns vs Best Practices in Rust & Axum

Direct comparisons illustrating common mistakes and their production-grade solutions.

---

## 1. Error Handling

### ❌ Antipattern: Panicking in Handlers with `.unwrap()`
```rust
// BAD: If the query fails, the entire request panics and can cause worker instability!
pub async fn get_persona(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let persona = sqlx::query_as::<_, Persona>("SELECT * FROM personas WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .unwrap(); // PANIC!

    Json(persona)
}
```

### ✅ Best Practice: Return Typed `AppResult<impl IntoResponse>`
```rust
// GOOD: Uses proper error propagation and maps missing records to 404
pub async fn get_persona(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<impl IntoResponse> {
    let persona = sqlx::query_as::<_, Persona>("SELECT * FROM personas WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Persona {} not found", id)))?;

    Ok(Json(json!({ "success": true, "persona": persona })))
}
```

---

## 2. Shared State Synchronization

### ❌ Antipattern: Locking an entire Async Mutex for in-memory reads
```rust
// BAD: Uses tokio::sync::Mutex to guard a read-heavy hashmap, forcing serial execution
pub struct AppState {
    pub cache: tokio::sync::Mutex<HashMap<i64, String>>,
}

pub async fn get_item(State(state): State<AppState>) -> impl IntoResponse {
    let lock = state.cache.lock().await; // Contention bottleneck!
    let item = lock.get(&1).cloned();
    Json(item)
}
```

### ✅ Best Practice: Read-Write Lock (`parking_lot::RwLock` or `ArcSwap`)
```rust
// GOOD: Allows hundreds of concurrent readers without blocking
pub struct AppState {
    pub cache: std::sync::Arc<parking_lot::RwLock<HashMap<i64, String>>>,
}

pub async fn get_item(State(state): State<AppState>) -> impl IntoResponse {
    let item = {
        let read_guard = state.cache.read();
        read_guard.get(&1).cloned()
    };
    Json(item)
}
```

---

## 3. Upstream HTTP Client Usage

### ❌ Antipattern: Constructing a new Client per request
```rust
// BAD: Reinitializes TLS context and connection pool every call!
pub async fn call_omniroute(url: &str) -> String {
    let client = reqwest::Client::new();
    client.get(url).send().await.unwrap().text().await.unwrap()
}
```

### ✅ Best Practice: Share a single pooled Client in `AppState`
```rust
// GOOD: Reuses TCP connections and keep-alive sockets
pub async fn call_omniroute(client: &reqwest::Client, url: &str) -> AppResult<String> {
    let resp = client.get(url).send().await?.text().await?;
    Ok(resp)
}
```

---

## 4. Large Streaming Relays

### ❌ Antipattern: Buffering 100MB into RAM before responding
```rust
// BAD: High memory usage; delays First-Byte-To-Client (TTFB)
pub async fn stream_llm(State(state): State<AppState>) -> impl IntoResponse {
    let mut resp = state.client.post("/chat").send().await.unwrap();
    let full_text = resp.text().await.unwrap(); // Holds entire response in RAM!
    full_text
}
```

### ✅ Best Practice: Stream byte chunks directly with `axum::response::sse`
```rust
// GOOD: Immediate First-Token latency; minimal RAM footprint
pub async fn stream_llm(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    let byte_stream = state.client.post("/chat").send().await?.bytes_stream();
    let sse_stream = async_stream::stream! {
        // Forward chunks as they arrive from upstream...
    };
    Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()))
}
```
