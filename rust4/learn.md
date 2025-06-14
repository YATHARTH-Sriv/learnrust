# 🧠 Things Learned In Chapter 4: Ownership and Memory in Rust

---

## 🪜 Stack and Heap Memory

* **Stack Memory**: Fast, efficient, uses LIFO (Last In First Out)
* **Heap Memory**: Slower, used for types like `String`, `Vec`
* Rust replaces garbage collection with the **ownership model** to manage memory efficiently.

---

## 🧵 Ownership Rules

![Ownership Rules](image.png)

---

## ✨ Literals vs `String::from("...")`

```rust
let s = "hello";                // Stored on Stack
let s = String::from("hello");  // Stored on Heap
```

---

## 🔄 Shallow Copy

![Shallow Copy](image-1.png)

---

## ⚡ Move Semantics (Not Shallow Copy)

![Move Semantics](image-2.png)

---

## 🔧 Deep Copy with `clone()`

![Deep Copy](image-3.png)

---

## ✔️ Copy Trait

![Copy Trait](image-4.png)

---

## 🤨 Clone Trait

![Clone Trait](image-5.png)

> The `Copy` trait is for types stored **only on the stack**.

---

## 🧱 Stack-Only Types Implement `Copy`

![Stack-Only Copy Types](image-6.png)

---

## 📌 References Without Ownership Transfer

* One **mutable** reference allowed
* Multiple **immutable** references allowed

![References](image-7.png)

---

## 💧 Mutable Referencing

```rust
let mut x = String::from("hello");

let r1 = &x;       // Immutable reference
let r2 = &mut x;   // Mutable reference ❌ Not allowed with r1
```

---

## ⚠️ Dangling Pointers Prevention

Rust prevents dangling pointers at **compile time**.

![Dangling Pointers](image-8.png)

---

## 📃 `.as_bytes()` and Byte Literals

```rust
let s = String::from("hello");
let bytes = s.as_bytes(); // &[u8]

let space = b' ';         // byte literal, value: 32
```

![Byte Details](image-9.png)
![Byte Logic](image-10.png)

---

## 📂 String Slices

* Use slices to work with **parts** of a `String`
* Avoids unnecessary cloning

![String Slices](image-11.png)

---


