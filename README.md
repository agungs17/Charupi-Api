# Charupi-Api

> **Modular Axum Service for Charupi**  
> Disusun dengan struktur modular terstandarisasi (**routes**, **handlers**, **middleware**, **config**, **error**, **state**) yang saat ini menjalankan endpoint `/health`.  
> Disertai skill komprehensif **`rust-axum-best-practices`** untuk referensi arsitektur dan performa saat pengembangan fitur backend lanjutan dimulai.

---

## Struktur Folder Modular

```
Charupi-Api/
├── Cargo.toml                  # Dependensi crate & release profile
├── Dockerfile                  # Multi-stage container build
├── docker-compose.yml          # Container orchestration
├── .env.example                # Template konfigurasi environment
├── src/
│   ├── main.rs                 # Server entry point & graceful shutdown
│   ├── lib.rs                  # Module exports
│   ├── config.rs               # Strongly-typed environment configs
│   ├── error.rs                # Standard AppError & IntoResponse
│   ├── state.rs                # Shared AppState
│   ├── routes/
│   │   ├── mod.rs              # Router aggregation
│   │   └── health.rs           # /health route
│   ├── handlers/
│   │   ├── mod.rs              # Handler exports
│   │   └── health.rs           # /health handler logic
│   └── middleware/
│       ├── mod.rs              # Middleware exports
│       ├── cors.rs             # Permissive CORS layer
│       └── logging.rs          # Tracing HTTP request logger
└── .agents/
    └── skills/
        └── rust-axum-best-practices/ # Comprehensive engineering skill
```

---

## Endpoint Aktif

| Method | Endpoint | Deskripsi |
| :--- | :--- | :--- |
| `GET` | `/health` | Pemeriksaan kesehatan layanan |

**Contoh Response:**
```json
{
  "status": "ok",
  "service": "charupi-api",
  "version": "0.1.0",
  "environment": "development"
}
```

---

## Menjalankan Server

```bash
# Debug mode
cargo run

# Release mode
cargo run --release

# Docker
docker compose up -d
```
