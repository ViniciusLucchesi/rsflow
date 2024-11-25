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
├── core/                  # The "core" of the application
│   ├── interfaces/        # Abstractions for repositories and databases
│   ├── models/            # Business entities like User
│   └── services/          # Domain logic, e.g., UserService
├── adapters/              # Infrastructure implementations
│   └── repositories/      # Concrete repository, e.g., InMemoryUserRepository
├── api/                   # HTTP layer (REST API)
└── main.rs                # Application entry point
```

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
- The **UserService** handles business logic and uses abstract **UserRepository** traits to interact with data.

### 🛠️ Adapter
- **InMemoryUserRepository** provides a concrete, fast, and lightweight implementation for testing and local development.

### 🌐 API
- RESTful routes expose core operations like creating, reading, updating, and deleting users.

---

## 🚧 Roadmap

- 🔄 Add a persistent database adapter (e.g., PostgreSQL).
- 🔐 Integrate authentication and authorization.
- 📜 Add more comprehensive tests (unit and integration).

