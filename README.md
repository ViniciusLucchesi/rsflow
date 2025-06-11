<div align=center>

# Hexagonal Architecture with Rust 🦀

[Features](#️-features) &nbsp;&bull;&nbsp; [Why Hexagonal?](#-why-hexagonal-architecture) &nbsp;&bull;&nbsp; [Project structure](#️-project-structure) &nbsp;&bull;&nbsp; [Usage](#-usage)

</div>


## 🛠️ Features

- **Hexagonal Architecture**: Domain-driven and modular, making it easy to swap components like databases or APIs! 💡
- **In-Memory User Database**: Fast and lightweight, perfect for testing and development. 🧠⚡
- **RESTful API**: Expose user operations over HTTP with clean, readable routes. 🌐
- **Type-Safe and Error-Proof**: Powered by Rust's blazing-fast safety guarantees. 🔒🔥
- **Dependency Injection**: Abstract repositories and services to keep the core logic decoupled from the infrastructure. 🛡️

---

## 🎯 Why Hexagonal Architecture?

Hexagonal Architecture focuses on keeping the **business logic** (the "core") independent of the **infrastructure** (like databases, APIs, etc.). This means:

1. 🧩 **Modular**: Swap out implementations easily.
2. 🛡️ **Testable**: Write unit tests for the core logic without worrying about the infrastructure.
3. 💪 **Resilient**: Changes in one area don't ripple unnecessarily through the entire codebase.

---

## 🏗️ Project Structure

Here's how the repository is structured:

```
src/
├── domain/                # Business entities and services
│   ├── models/            # Core data structures
│   └── services/          # Domain logic
├── ports/                 # Traits used as application ports
│   └── database/          # Repository interfaces
├── adapters/              # Implementations of ports
│   ├── api/               # HTTP layer (REST API)
│   └── repositories/      # e.g., InMemoryUserRepository
└── main.rs                # Application entry point
```

Domain services depend only on the traits defined in `ports`. Adapters implement
these traits and are injected at runtime, enabling easy swapping of
implementations.

---

## 🌟 Usage

### 🛴 Getting Started

1. Run the server:
   ```bash
   cargo run
   ```

2. Hit the API:
   - Create a user:
     ```bash
     curl -X POST http://localhost:3000/users -H "Content-Type: application/json" \
     -d '{"name": "Alice", "email": "alice@example.com"}'
     ```
   - Get a user:
     ```bash
     curl http://localhost:3000/users/<user_id>
     ```
   - Get all users:
     ```bash
     curl http://localhost:3000/all-users
     ```

---

## 📚 How It Works

### 👩‍💻 Core Logic
- The `UserService` trait defines operations for managing users.
- `UserServiceImpl` implements this trait and depends on a repository that
  implements `UserRepository`.

### 🛠️ Adapter
- **InMemoryUserRepository** provides a concrete, fast, and lightweight
  implementation for testing and local development.
  Different repositories can be injected in `main.rs` by creating another
  struct that implements `UserRepository` and passing it to `UserServiceImpl::new`.

### 🌐 API
- RESTful routes expose core operations like creating, reading, updating, and deleting users.
  Handlers receive a boxed `UserService` trait object, allowing different service
  implementations to be injected without changing the HTTP layer.

---

## 🚧 Roadmap

- 🔄 Add a persistent database adapter (e.g., PostgreSQL).
- 🔐 Integrate authentication and authorization.
- 📜 Add more comprehensive tests (unit and integration).

