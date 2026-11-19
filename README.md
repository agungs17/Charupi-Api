# Charupi-Api

> **High-Performance Rust Backend Gateway & Business Logic Service for Charupi**  
> Dibangun dengan **Axum**, **Tokio**, **SQLx**, dan **Tower-HTTP** untuk bertindak sebagai perantara cerdas antara klien **Charupi-App** dan gateway LLM **OmniRoute**.

---

## Fitur Utama

1. **High-Throughput & Low Latency**:
   - Zero-cost async runtime menggunakan `tokio` multi-threaded engine.
   - Non-blocking I/O dan connection pooling cerdas (`reqwest` + `sqlx`).
2. **Server-Sent Events (SSE) Streaming Relay**:
   - Meneruskan token respons LLM dari OmniRoute ke aplikasi mobile secara instan tanpa buffering memori besar di backend.
   - Menyimpan respons lengkap ke database lokal secara otomatis setelah stream selesai.
3. **Database Relasional & Vector Storage**:
   - Menggunakan SQLite dengan mode **WAL (Write-Ahead Logging)**, cache memori 64MB, dan foreign keys aktif.
   - Menampung tabel `personas`, `user_persona`, `story_characters`, `story_events`, `messages`, dan `chunks`.
   - Vektor pencarian semantik (RAG) berbasis *cosine distance*.
4. **Prompt Engineering & Anti-Hijacking Guard**:
   - Merakit system prompt dengan tag semantik XML (`<world_and_characters>`, `<story_context>`, `<memory_context>`, `<user_profile>`, `<critical_reminder>`).
   - Menyisipkan anchor reminder untuk menjaga kepatuhan format dialog.
5. **Observabilitas & Keandalan**:
   - Structured tracing via `tracing-subscriber`.
   - Standarisasi error response JSON berbasis `AppError` enum.
   - Graceful shutdown listener (Ctrl+C dan SIGTERM).

---

## Struktur Direktori

```
Charupi-Api/
├── Cargo.toml                  # Konfigurasi crate & profile release
├── Dockerfile                  # Multi-stage caching build
├── docker-compose.yml          # Container orchestration
├── .env.example                # Template variabel lingkungan
├── src/
│   ├── main.rs                 # Server entry point & graceful shutdown
│   ├── lib.rs                  # Module root exports
│   ├── config.rs               # Strongly-typed environment configs
│   ├── error.rs                # AppError enum & IntoResponse handler
│   ├── state.rs                # Shared AppState (DB, HTTP client, Config)
│   ├── db/
│   │   ├── pool.rs             # SQLite connection pool (WAL mode)
│   │   └── migrations.rs       # Skema database & table initialization
│   ├── models/                 # Domain entities & DTO schemas
│   ├── services/
│   │   ├── omniroute_client.rs # HTTP Client & SSE stream reader
│   │   ├── prompt_service.rs   # XML Prompt construction
│   │   └── vector_service.rs   # Cosine distance & vector retrieval
│   ├── handlers/               # Route handlers (chat, story, persona, memory)
│   ├── routes/                 # Modular API endpoints
│   └── middleware/             # Trace logging, CORS, timeout, limits
└── .agents/
    └── skills/
        └── rust-axum-best-practices/ # Comprehensive engineering skill
```

---

## Menjalankan Server

### 1. Prasyarat
- Rust Toolchain (`rustc` & `cargo` 1.80+)
- Server OmniRoute aktif (default: `http://127.0.0.1:20128/v1`)

### 2. Konfigurasi
Salin file template `.env.example` menjadi `.env`:
```bash
cp .env.example .env
```

### 3. Build & Run Lokal
```bash
# Debug mode
cargo run

# Release mode (teroptimasi penuh)
cargo run --release
```

### 4. Menjalankan via Docker
```bash
docker compose up -d
```

---

## Endpoint API

| Method | Endpoint | Keterangan |
| :--- | :--- | :--- |
| `GET` | `/health` | Pemeriksaan kesehatan server dan database |
| `GET` | `/api/v1/personas` | Mengambil seluruh daftar Story Room |
| `POST` | `/api/v1/personas` | Membuat Story Room baru |
| `GET` | `/api/v1/personas/:id` | Detail Story Room |
| `PUT` | `/api/v1/personas/:id` | Memperbarui Story Room |
| `DELETE`| `/api/v1/personas/:id` | Menghapus Story Room |
| `GET` | `/api/v1/user-personas` | Mengambil profil user persona |
| `POST` | `/api/v1/user-personas` | Menambah profil user persona |
| `GET` | `/api/v1/story/characters/:persona_id` | Karakter dalam suatu Story Room |
| `POST` | `/api/v1/story/characters` | Menambah karakter cerita baru |
| `GET` | `/api/v1/story/events/:persona_id` | Garis waktu peristiwa cerita |
| `POST` | `/api/v1/story/events` | Menambah peristiwa cerita baru |
| `GET` | `/api/v1/chat/messages/:persona_id` | Riwayat pesan obrolan |
| `POST` | `/api/v1/chat/send` | Mengirim pesan (mendukung SSE Stream / JSON) |
| `DELETE`| `/api/v1/chat/clear/:persona_id` | Membersihkan riwayat pesan |
| `POST` | `/api/v1/memory/search` | Pencarian memori vektor relevan |
| `DELETE`| `/api/v1/memory/clear/:persona_id` | Membersihkan memori vektor persona |
