# Charupi-Api

> **Modular Axum Service for Charupi**  
> Disusun dengan struktur modular terstandarisasi (**routes**, **handlers**, **middleware**, **config**, **error**, **state**) yang saat ini menjalankan endpoint `/health`.  
> Disertai skill komprehensif **`rust-axum-best-practices`** untuk referensi arsitektur dan performa saat pengembangan fitur backend lanjutan dimulai.

[![Swagger API Documentation](https://img.shields.io/badge/Swagger-API%20Documentation-85EA2D?style=for-the-badge&logo=swagger&logoColor=black)](https://agungs17.github.io/Charupi-Api/)

---

## 📖 API Documentation (Swagger UI)

Dokumentasi API interaktif berbasis Swagger / OpenAPI 3.0 sudah disiapkan dan di-host via GitHub Pages:

👉 **[📖 Buka Swagger API Documentation](https://agungs17.github.io/Charupi-Api/)**

Melalui halaman Swagger tersebut, Anda dapat melihat spesifikasi parameter, model response, serta menguji request secara live menggunakan fitur **Try it out**.

---

## Struktur Folder Modular

```
Charupi-Api/
├── Cargo.toml                  # Dependensi crate & release profile
├── Dockerfile                  # Multi-stage container build
├── docker-compose.yml          # Container orchestration
├── .env.example                # Template konfigurasi environment
├── docs/                       # Swagger UI untuk GitHub Pages
│   ├── index.html              # Swagger UI web standalone
│   └── openapi.yaml            # Spesifikasi OpenAPI 3.0.3
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
