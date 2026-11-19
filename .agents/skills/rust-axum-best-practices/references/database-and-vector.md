# Database & Vector Operations with SQLx & SQLite

Charupi-Api relies on an embedded, zero-maintenance, high-performance database setup using **SQLite** managed via **SQLx**. Follow these configurations to ensure database operations never bottleneck the server.

---

## 1. SQLite High-Concurrency Tuning (WAL Mode)

By default, SQLite locks the entire database file during writes. To achieve high concurrency where readers never block writers and writers never block readers, **Write-Ahead Logging (WAL)** mode is mandatory.

### Essential PRAGMA Directives:
```rust
let connect_opts = SqliteConnectOptions::from_str(&config.database_url)?
    .create_if_missing(true)
    .journal_mode(SqliteJournalMode::Wal)          // Write-Ahead Logging
    .synchronous(SqliteSynchronous::Normal)        // Safe and significantly faster in WAL mode
    .foreign_keys(true)                            // Enforce referential integrity
    .busy_timeout(Duration::from_secs(10));        // Wait up to 10s if database is busy

// Additional in-memory caching:
sqlx::query("PRAGMA cache_size = -64000;").execute(&pool).await?; // 64MB memory page cache
sqlx::query("PRAGMA temp_store = MEMORY;").execute(&pool).await?; // Temporary tables in RAM
```

---

## 2. Pool Sizing Rules

- SQLite is an embedded single-file database. Having hundreds of connections will **not** increase write throughput and may trigger file lock contention (`database is locked`).
- **Optimal max connections:** Between `8` and `16` for read-heavy workloads.
- **Acquire Timeout:** Set an acquire timeout (e.g. `5s`) on the pool so pending requests fail fast with a proper error instead of hanging indefinitely.

---

## 3. High-Performance Vector Cosine Operations

Vector similarity search is crucial for RAG memory retrieval.

### Cosine Distance Calculation in Rust:
```rust
pub fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 1.0;
    }

    let mut dot_product = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;

    for (x, y) in a.iter().zip(b.iter()) {
        dot_product += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }

    if norm_a <= 0.0 || norm_b <= 0.0 {
        return 1.0;
    }

    let similarity = dot_product / (norm_a.sqrt() * norm_b.sqrt());
    (1.0 - similarity).clamp(0.0, 2.0)
}
```

### Storing Vectors in SQLite BLOBs:
Convert `Vec<f32>` to compact little-endian byte slices (`Vec<u8>`) before storing in SQLite:
```rust
// 768 dimensions * 4 bytes = 3,072 bytes per chunk
pub fn f32_to_bytes(floats: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(floats.len() * 4);
    for &val in floats {
        bytes.extend_from_slice(&val.to_le_bytes());
    }
    bytes
}
```

---

## 4. Query Indexing Best Practices

- Always index foreign keys and columns frequently used in `WHERE` and `ORDER BY` clauses:
  - `CREATE INDEX idx_messages_persona ON messages(persona_id, id DESC);`
  - `CREATE INDEX idx_chunks_persona_last ON chunks(persona_id, last_msg_id);`
- Avoid `SELECT *` in high-throughput loops if only 2 columns are needed.
- Use `fetch_optional` instead of `fetch_one` when the record may legitimately not exist, handling `Option::None` with `AppError::NotFound`.
