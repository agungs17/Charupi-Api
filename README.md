# Charupi-Api

> **Modular Backend Service for Charupi**  
> High-performance backend gateway and service built with Rust, Axum, Tokio, and Tower-HTTP.  
> Designed with clean separation of concerns and accompanied by the comprehensive **`rust-axum-best-practices`** skill.

[![Scalar API Documentation](https://img.shields.io/badge/Scalar-API%20Reference-8A2BE2?style=for-the-badge&logo=openapi-initiative&logoColor=white)](http://localhost:8080/docs/)

---

## 📖 API Documentation

Interactive API documentation powered by **Scalar** with built-in Dark Mode and multi-language code examples.

### How to Access:

Start the server, then open your browser at:  
👉 **[http://localhost:8080/docs/](http://localhost:8080/docs/)**

---

## Project Structure

```
Charupi-Api/
├── docs/       # Interactive API reference and OpenAPI specifications
├── src/        # Application source code
│   ├── handlers/   # HTTP request handlers
│   ├── middleware/ # Request logging and CORS layers
│   └── routes/     # Route declarations and endpoints
└── .agents/    # Engineering guidelines and best-practice skills
```

---

## Getting Started

### Prerequisites
- [Rust toolchain](https://www.rust-lang.org/tools/install) (Edition 2021, Rust 1.80+)

### Running the Server

```bash
# Development mode (Hot Reload with auto-check & auto-restart)
cargo dev

# Normal run
cargo run

# Release mode (optimized)
cargo run --release

# Run with Docker Compose
docker compose up -d
```
