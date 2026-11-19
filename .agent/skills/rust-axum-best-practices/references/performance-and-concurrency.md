# Performance & Concurrency in Rust & Tokio

Rust delivers near-hardware performance, but improper async patterns or runtime blocking can severely degrade latency and throughput. Follow these principles to achieve maximum efficiency.

---

## 1. Never Block the Async Executor

Tokio operates an M:N green thread scheduler over a fixed worker thread pool (typically equal to the number of CPU cores). If an async task runs synchronous I/O or heavy computation on a worker thread, all other async tasks assigned to that thread are stalled.

### Anti-Pattern (Blocking Worker Thread):
```rust
// BAD: Blocks the entire Tokio async worker thread!
async fn compute_heavy_vectors(data: Vec<f32>) -> Vec<f32> {
    std::thread::sleep(Duration::from_millis(50)); // Sync blocking!
    expensive_matrix_multiplication(&data)
}
```

### Best Practice (Using `spawn_blocking`):
```rust
// GOOD: Offloads heavy computation to Tokio's dedicated blocking thread pool
async fn compute_heavy_vectors(data: Vec<f32>) -> AppResult<Vec<f32>> {
    tokio::task::spawn_blocking(move || {
        expensive_matrix_multiplication(&data)
    })
    .await
    .map_err(|e| AppError::Internal(format!("Task failed: {}", e)))
}
```

---

## 2. Mutex Selection: Async vs Synchronous

Choosing the wrong mutex is one of the most common performance bottlenecks in Rust backends.

| Mutex Type | When to Use | Trade-offs |
| :--- | :--- | :--- |
| `parking_lot::Mutex` or `std::sync::Mutex` | Protecting in-memory data for short operations (< 1µs) **without `.await` inside the lock**. | Fastest. Lowest overhead. Zero heap allocation. |
| `parking_lot::RwLock` | Read-heavy in-memory caches where writes are rare. | High concurrency for readers. |
| `tokio::sync::Mutex` | **Only** when a mutex guard must be held across an `.await` boundary (e.g. reserving a network channel). | Higher overhead. May cause lock contention if held too long. |

---

## 3. Minimizing Allocations & Clones

1. **Pre-allocate Collections**: When the size is known or estimable, use `Vec::with_capacity(n)` and `String::with_capacity(n)` to prevent repeated reallocation and copying.
2. **Use String Slices (`&str`)**: Avoid converting string slices into owned `String` until ownership is genuinely required.
3. **Use Bytes Buffer Pools**: When forwarding byte streams from upstream LLMs, forward `bytes::Bytes` directly without converting to `String` and back.

---

## 4. Release Profile Optimization (`Cargo.toml`)

Ensure production builds utilize aggressive compilation optimizations:

```toml
[profile.release]
opt-level = 3          # Maximum optimizations
lto = "thin"           # Link-Time Optimization across crates
codegen-units = 1      # Reduces parallel code generation for better optimizations
panic = "abort"        # Removes stack unwinding code, reducing binary size
strip = true           # Strips symbols and debug info from final binary
```

---

## 5. High-Performance Memory Allocator (Optional)

On Linux/macOS production servers with heavy multi-threaded allocation loads, switching from the system allocator to `mimalloc` or `jemalloc` yields 10%–25% throughput improvements and reduces fragmentation:

```rust
// In main.rs (optional):
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
```
