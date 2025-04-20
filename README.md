# 🧠 Plugin Orchestrator · Built in Rust · Mission-Aligned Infrastructure

![Build](https://img.shields.io/badge/build-passing-brightgreen)
![License](https://img.shields.io/badge/license-MIT-blue)
![Made with Rust](https://img.shields.io/badge/made%20with-rust-orange)
![Dockerized](https://img.shields.io/badge/docker-ready-blue)
![Secure](https://img.shields.io/badge/security-scoped%20execution-critical)

---

### 🔧 Overview

**Plugin Orchestrator** is a scoped plugin execution engine written in Rust, designed to reflect the real-world architectural principles of:

- 🧠 **OpenAI** → secure, AI-aligned plugin orchestration  
- 🛰️ **Anduril** → field-grade modular tooling with edge-safe fallbacks  
- 📡 **Palantir** → structured data flow, policy enforcement, and scoped intelligence

This is **signal-driven infrastructure**, not a toy project.  
It's designed for **real agents, real tools, and real missions.**

---

### 🚀 Features

- ✅ **Scoped plugin execution** (`plugin:xyz`) with deny-by-default access control  
- 🧩 **Modular plugin architecture** via trait-based dynamic execution (`PluginExecutor`)  
- 🔄 **REST interface** with `/execute` POST endpoint (Axum, JSON payloads)  
- 🔐 **Panic-safe sandboxing** using `.catch_unwind()` with `futures`  
- 🐳 **Dockerized static binary** (built with Alpine + MUSL)  
- 📦 **MIT licensed** – production ready, remixable, royalty-free

---

### 📦 Example Plugins

| Plugin | Description |
|--------|-------------|
| `echo` | Repeats back your message |
| `time` | Returns current UTC timestamp |

More plugins can be added easily by implementing the `PluginExecutor` trait.

---

### 📥 Run It Now

#### 🔧 Docker Build & Run

```bash
# Build image
docker build -t plugin-orchestrator .

# Run it
docker run -p 4000:4000 --env-file .env plugin-orchestrator
```

#### 🔁 Sample Request (via curl or Postman)

```json
POST /execute
{
  "plugin": "echo",
  "payload": { "message": "Still good?" },
  "scopes": ["plugin:echo"]
}
```

Returns:
```json
{
  "output": "You said: Still good?"
}
```

---

### 🧬 Developer Setup (Rust)

```bash
cargo run
```

Access at [http://localhost:4000/execute](http://localhost:4000/execute)

---

### ⚖️ License

MIT — free to use, modify, and deploy.

---

### 👤 Author

```text
Built by Michael Alexander Montoya  
📫 montoyamichael973@gmail.com  
Third-generation American. Systems builder. Silent operator.
```

---

### 🧠 Part of the `cureprotocol` project

> Signal-grade infrastructure for plugin orchestration, agent interfaces, and strategic AI tooling.

Explore more at: [https://github.com/cureprotocol](https://github.com/cureprotocol)

---
