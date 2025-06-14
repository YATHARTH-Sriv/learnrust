# 🚀 Things Learned in Chapter 1: Cargo Basics in Rust

---

## 🛠️ Essential Cargo Commands

### 📦 `cargo init`

Initializes a new Rust project with the basic structure:

```bash
cargo init
```

> 🗂️ Creates a `main.rs` file and `Cargo.toml`.

---

### 🔨 `cargo build`

Compiles your Rust project in **debug mode** by default:

```bash
cargo build
```

> ⚙️ Creates an executable inside `target/debug/`.

---

### 🚀 `cargo run`

Builds and **executes** your project in one step:

```bash
cargo run
```

> 🏁 Useful during rapid development and testing.

---

### 🧪 `cargo check`

Checks if your code compiles **without producing an executable**:

```bash
cargo check
```

> ⚡ Super fast — perfect for catching syntax and type errors quickly.

---

### 📈 `cargo build --release`

Builds the project in **release mode**:

```bash
cargo build --release
```

> 🚄 Optimized for performance:
>
> * Removes dead code
> * Inlines functions
> * Other compiler-level enhancements

> 🐢 Slower to build but great for final production code.

---

## ✅ Summary

| Command                 | Purpose                            | Output                  |
| ----------------------- | ---------------------------------- | ----------------------- |
| `cargo init`            | Initializes a project              | `main.rs`, `Cargo.toml` |
| `cargo build`           | Builds in debug mode               | `target/debug/`         |
| `cargo run`             | Builds and runs                    | Executes binary         |
| `cargo check`           | Compiles without generating binary | Syntax/type checking    |
| `cargo build --release` | Builds optimized production binary | `target/release/`       |

---
