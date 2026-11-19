# Charupi-Api

> **Modular Axum Service for Charupi**  
> Built with a standardized modular architecture (**routes**, **handlers**, **middleware**, **config**, **error**, **state**) running on Rust Axum, with the `/health` endpoint enabled.  
> Accompanied by the comprehensive **`rust-axum-best-practices`** skill for architectural guidance and high-performance backend engineering.

[![Scalar API Documentation](https://img.shields.io/badge/Scalar-API%20Reference-8A2BE2?style=for-the-badge&logo=openapi-initiative&logoColor=white)](http://localhost:8080/docs/)

---

## 📖 API Documentation (Scalar Interactive Reference)

Interactive API documentation powered by **Scalar** (a modern OpenAPI reference featuring built-in Dark Mode and automatic multi-language code snippets).

### How to Access:

Start the server (`cargo run`), then open your browser at:  
👉 **[http://localhost:8080/docs/](http://localhost:8080/docs/)**

---

## Modular Project Structure

```
Charupi-Api/
├── Cargo.toml                  # Crate dependencies (Axum, Tokio, Tower-HTTP)
├── Dockerfile                  # Multi-stage container build
├── docker-compose.yml          # Container orchestration
├── .env.example                # Environment configuration template
├── docs/                       # Scalar API Reference
│   ├── index.html              # Scalar web client (@scalar/api-reference)
│   └── openapi.yaml            # OpenAPI 3.0.3 specification
├── src/
│   ├── main.rs                 # Server entry point & graceful shutdown
│   ├── lib.rs                  # Module root exports
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

## Running the Server

```bash
# Debug mode
cargo run

# Release mode (fully optimized)
cargo run --release

# Run with Docker Compose
docker compose up -d
```
