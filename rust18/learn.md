# 🦀 Chapter 18: OOPs in Rust

This README summarizes Chapter 18 of the Rust Book, which discusses object-oriented programming (OOP) principles and how they apply in Rust. Rust does not follow traditional OOP patterns exactly, but it supports many OOP principles in idiomatic ways.

---

## 📦 What Is Object-Oriented Programming?

Object-Oriented Programming (OOP) is a paradigm where programs are built using "objects" that encapsulate data and behavior. Rust blends multiple paradigms, and while it does not offer inheritance like classic OOP languages, it supports:

* Objects (via structs/enums + methods)
* Encapsulation (via modules and privacy)
* Polymorphism (via traits and generics)

---

## 📌 Characteristics of OOP in Rust

### 1. **Objects = Data + Behavior**

Rust supports this through:

* `struct` and `enum` types for data
* `impl` blocks to attach methods

```rust
struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}
```

This mirrors the "object = data + methods" idea.

---

### 2. **Encapsulation**

Encapsulation means hiding internal state and exposing only what’s necessary. Rust supports this using the `pub` keyword.

Example:

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        if let Some(_) = result {
            self.update_average();
        }
        result
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

Fields are private; the user can’t manipulate them directly, ensuring consistency.

---

### 3. **Inheritance**

Rust **does not** support traditional inheritance. Instead, Rust encourages:

* **Composition**: build types from other types.
* **Traits**: define shared behavior.
* **Default trait methods**: reuse behavior across types.

```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

---

### 4. **Polymorphism**

Rust supports polymorphism through:

* **Generics** (compile-time)
* **Trait objects** (runtime)

This is called **bounded parametric polymorphism**.

Example:

```rust
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

---

## ⚖️ Trade-offs

* No inheritance = more flexible and safer designs.
* Composition and traits encourage **explicitness** and **modularity**.
* Prevents common problems with deep inheritance chains.

---

## ✅ Summary Table

| OOP Concept   | Rust Equivalent                         |
| ------------- | --------------------------------------- |
| Object        | Struct + Impl                           |
| Encapsulation | Module system + `pub`                   |
| Inheritance   | ❌ (Use traits + composition)            |
| Polymorphism  | Generics + Trait Bounds / Trait Objects |

---

