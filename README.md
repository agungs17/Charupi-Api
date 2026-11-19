# Charupi-Api

> **Modular Axum Service for Charupi**  
> Disusun dengan struktur modular terstandarisasi (**routes**, **handlers**, **middleware**, **config**, **error**, **state**) yang saat ini menjalankan endpoint `/health`.  
> Disertai skill komprehensif **`rust-axum-best-practices`** untuk referensi arsitektur dan performa saat pengembangan fitur backend lanjutan dimulai.

[![Scalar API Documentation](https://img.shields.io/badge/Scalar-API%20Reference-8A2BE2?style=for-the-badge&logo=openapi-initiative&logoColor=white)](http://localhost:8080/docs/)

---

## 📖 API Documentation (Scalar Interactive Reference)

Dokumentasi API interaktif menggunakan **Scalar** (modern OpenAPI reference dengan Dark Mode & multi-language code snippets otomatis).

### Cara Mengakses:

1. **Secara Lokal (Direct dari Server Rust):**
   Cukup jalankan server (`cargo run`), lalu buka browser ke:  
   👉 **[http://localhost:8080/docs/](http://localhost:8080/docs/)**

2. **Secara Online (GitHub Pages / Cloudflare Pages):**  
   👉 **[https://agungs17.github.io/Charupi-Api/](https://agungs17.github.io/Charupi-Api/)**

---

## Struktur Folder Modular

```
Charupi-Api/
├── Cargo.toml                  # Dependensi crate (Axum, Tokio, Tower-HTTP)
├── Dockerfile                  # Multi-stage container build
├── docker-compose.yml          # Container orchestration
├── .env.example                # Template konfigurasi environment
├── docs/                       # Scalar API Reference
│   ├── index.html              # Scalar web client (@scalar/api-reference)
│   └── openapi.yaml            # Spesifikasi OpenAPI 3.0.3
├── src/
│   ├── main.rs                 # Server entry point & graceful shutdown
│   ├── lib.rs                  # Module exports
│   ├── config.rs               # Strongly-typed environment configs
│   ├── error.rs                # Standard AppError & IntoResponse
│   ├── state.rs                # Shared AppState
│   ├── routes/
│   │   ├── mod.rs              # Router aggregation (/health & /docs)
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
| `GET` | `/docs/` | Dokumentasi API interaktif Scalar |

**Contoh Response `/health`:**
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
