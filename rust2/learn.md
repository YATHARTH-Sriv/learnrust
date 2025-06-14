# 📘 Things Learned In Chapter 2: Rust Essentials

---

## ✨ Prelude in Rust

> **What is a Prelude?**

Rust prelude brings commonly used items into scope automatically so you don’t have to import them explicitly every time.

```rust
use tokio::prelude::*; // Brings all prelude traits and functions into scope
```

---

## 🧭 References

Rust uses **references** (`&`) to let code access data **without taking ownership**:

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);
```

---

## 🔁 Mutability

```rust
let mut x = 5;
x = 6; // ✅ Works!
```

> Variables are immutable by default. Use `mut` to allow mutation.

---

## 🛠️ Binary vs Library Crates

Rust projects can be either of the following:

### 📦 Binary Crate

* Contains a `main()` function.
* Compiles to an executable.
* Created with:

```bash
cargo new my_app
```

### 📚 Library Crate

* No `main()` function.
* Meant to be reused across projects.
* Created with:

```bash
cargo new my_utils --lib
```

---

## 📄 Documentation on Crates

Generate and view docs for your crate and its dependencies:

```bash
cargo doc --open
```

> 📚 Opens beautiful HTML documentation in your browser.

---

## 🌀 Shadowing

Shadowing lets you redeclare a variable with the same name:

```rust
let x = 5;
let x = x + 1; // Shadows previous x
```

> 🌟 Unlike `mut`, shadowing creates a **new variable**, not changing the original.

---

