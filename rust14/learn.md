# 🧠 Things Learned In Chapter 14: Cargo and Crates in Rust

## ⚙️ Building Your Crate

```bash
cargo build              # Compile the project in debug mode
cargo build --release   # Compile with optimizations
```

### ⚖️ Optimization Levels

```toml
[profile.dev]
opt-level = 0            # No optimizations (default)

[profile.release]
opt-level = 3            # Maximum optimizations
```

## 📃 Documentation for Your Crate

Write docs in your `.rs` files using `///` for items or `//!` for modules:

````rust
/// Adds two numbers.
///
/// # Examples
///
/// ```
/// let sum = my_crate::add(2, 3);
/// assert_eq!(sum, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
````

Then generate HTML documentation with:

```bash
cargo doc --open
```

### 📄 Commonly Used Sections in Docs

* `# Examples`: How to use the function
* `# Panics`: When the function could panic
* `# Errors`: Describe possible `Result` errors
* `# Safety`: When using `unsafe` code, document the contract

## 💡 Documentation Tests

Run tests embedded in your docs:

```bash
cargo test
```

This will also run the examples in your `///` doc comments.

---

## 🚀 Creating and Publishing Your Crate

1. Create an account on [crates.io](https://crates.io)
2. Get your API key from your account settings
3. Log in:

```bash
cargo login <your-api-key>
```

4. Update `Cargo.toml`:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

5. Push your code and publish:

```bash
git push origin main
cargo publish
```

---

## ✨ Exporting a Convenient Public API with `pub use`

You can re-export internal items to simplify your public API:

```rust
// src/lib.rs
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds;
pub mod utils;
```

### Folder Structure

```
art/
├── Cargo.toml
└── src/
    ├── lib.rs          # main entry point
    ├── kinds.rs        # defines PrimaryColor and SecondaryColor enums
    └── utils.rs        # defines mix function
```

### src/kinds.rs

```rust
pub enum PrimaryColor {
    Red,
    Yellow,
    Blue,
}

pub enum SecondaryColor {
    Orange,
    Green,
    Purple,
}
```

### src/utils.rs

```rust
use crate::kinds::*;

pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
    SecondaryColor::Orange // example stub
}
```

### Why `pub mod` and `pub use`?

* `pub mod kinds;` loads and exposes the module.
* `pub use self::kinds::PrimaryColor;` brings the type to crate root for easier access.

So users can now write:

```rust
use art::PrimaryColor;
use art::mix;
```

instead of:

```rust
use art::kinds::PrimaryColor;
use art::utils::mix;
```

---

## 📚 Workspaces

### Creating a Workspace

1. Create a folder:

```bash
mkdir my_workspace
cd my_workspace
```

2. Add a top-level `Cargo.toml` with a workspace section:

```toml
[workspace]
members = ["adder", "add_one"]
resolver = "2"
```

3. Initialize member crates:

```bash
cargo init adder --bin
cargo init add_one --lib
```

### Use Items from Other Crates in the Workspace

In the binary crate’s `Cargo.toml`:

```toml
[dependencies]
add_one = { path = "../add_one" }
```

Then in code:

```rust
use add_one::add_one;
```

### Building and Running

* Build all crates:

```bash
cargo build
```

* Builds to a shared `target/` folder at workspace root.
* Run a specific binary package:

```bash
cargo run -p adder
```

## 📊 Depending on an External Package in a Workspace

* You can add external dependencies in the top-level `Cargo.toml` under `[workspace.dependencies]`:

```toml
[workspace.dependencies]
rand = "0.8"
```

* This centralizes the version and avoids version conflicts.
* However, **you still need to declare `rand` in each crate’s own `[dependencies]`** to use it.

The workspace setting ensures all crates use the same version, but it doesn't automatically import it.

---

## ✅ Tests in Workspaces

* Run all tests in all member crates:

```bash
cargo test
```

* Run tests for a specific crate:

```bash
cargo test -p add_one
```

---

## 👉 Installing Binaries

To install a binary from your project (like for local use or global path):

```bash
cargo install --path .
```

