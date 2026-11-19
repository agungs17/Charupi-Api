# Networking, HTTP Clients & Streaming in Axum

Charupi-Api acts as a reverse proxy and business logic engine between mobile clients and the OmniRoute LLM gateway. Low latency and reliable streaming are critical.

---

## 1. Reqwest Client Configuration (Connection Pooling)

Never instantiate a new `reqwest::Client` per request. Creating a client initializes new connection pools, TLS sessions, and socket pools, which destroys performance.

### Optimal Client Initialization:
```rust
let http_client = reqwest::Client::builder()
    .pool_idle_timeout(Duration::from_secs(90))
    .pool_max_idle_per_host(10)
    .tcp_nodelay(true)                                // Disable Nagle's algorithm for instant packet flush
    .timeout(Duration::from_secs(120))                // Default safety timeout
    .build()?;
```
Pass this single `Client` instance across your application via `AppState`.

---

## 2. Server-Sent Events (SSE) Streaming Relay

When an LLM generates tokens over Server-Sent Events, the backend must stream tokens directly down to the mobile client with minimal buffer overhead.

### Architectural Flow:
```
[ OmniRoute LLM Gateway ]
           │
           │ (HTTP Chunked Stream)
           ▼
[ Charupi-Api: byte_stream ]
           │
           ├─► 1. Forward raw delta chunks immediately to mobile client (SSE)
           │
           └─► 2. Accumulate full response in memory buffer
                     │
                     ▼
          [ Once Stream Completes: INSERT INTO messages (assistant) ]
```

### Axum SSE Handler Implementation:
```rust
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::StreamExt;

let sse_stream = async_stream::stream! {
    let mut stream = upstream_bytes_stream;
    let mut full_accumulated = String::new();

    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(bytes) => {
                if let Ok(text) = std::str::from_utf8(&bytes) {
                    // 1. Yield to mobile client immediately
                    yield Ok::<_, Infallible>(Event::default().data(text));

                    // 2. Extract delta content for persistence
                    // ...
                }
            }
            Err(err) => {
                tracing::error!(?err, "Error in upstream stream");
                break;
            }
        }
    }

    // 3. Persist complete assistant reply to database
    if !full_accumulated.is_empty() {
        let _ = save_assistant_message(&pool, persona_id, &full_accumulated).await;
    }
};

Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()).into_response())
```

---

## 3. Backpressure & Client Disconnection Handling

- When a client abruptly closes the app or cancels a request, the TCP socket is dropped.
- Axum detects broken connections when `yield` fails or when the request `AbortSignal` / cancel token triggers.
- Clean up any active upstream HTTP calls immediately to avoid wasting LLM API tokens.
