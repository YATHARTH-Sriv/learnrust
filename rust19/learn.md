# 🚀 Chapter 19: Pattern Matching

Patterns in Rust are used in many different places and forms. You’ve already been using them—often without realizing it! This chapter explores the various locations and situations where patterns appear in Rust code.

---

## 🧠 What Is a Pattern?

A **pattern** in Rust is used to match values and bind variables. It can appear in many parts of the language including `match` arms, `if let`, `while let`, `for` loops, `let` bindings, and function parameters.

---

## 🎯 1. `match` Arms

A `match` expression requires patterns in its arms and must be exhaustive.

```rust
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

Catch-all pattern `_` is often used to handle "all other" cases.

---

## ❓ 2. `if let` and `else if let` Expressions

Allows conditional pattern matching for a single case and can mix with `else if`.

```rust
let favorite_color: Option<&str> = None;
let is_tuesday = false;
let age: Result<u8, _> = "34".parse();

if let Some(color) = favorite_color {
    println!("Using your favorite color: {color}");
} else if is_tuesday {
    println!("Tuesday is green day!");
} else if let Ok(age) = age {
    if age > 30 {
        println!("Using purple as the background color");
    } else {
        println!("Using orange as the background color");
    }
} else {
    println!("Using blue as the background color");
}
```

> ✅ `if let` is convenient but not exhaustive—unlike `match`.

---

## 🔀 3. `while let` Loops

Keeps looping while the pattern matches.

```rust
let (tx, rx) = std::sync::mpsc::channel();

std::thread::spawn(move || {
    for val in [1, 2, 3] {
        tx.send(val).unwrap();
    }
});

while let Ok(value) = rx.recv() {
    println!("{value}");
}
```

---

## 🗖 4. `for` Loops

In `for x in y`, `x` is a pattern. Useful for destructuring:

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{value} is at index {index}");
}
```

---

## ✨ 5. `let` Statements

Every `let` uses a pattern:

```rust
let x = 5; // pattern is x
```

Destructuring with tuples:

```rust
let (x, y, z) = (1, 2, 3);
```

Mismatch leads to compiler error:

```rust
let (x, y) = (1, 2, 3); // error: expected 3-element tuple, found 2
```

---

## 👨‍💻 6. Function Parameters

Function parameters are also patterns:

```rust
fn foo(x: i32) {
    // x is a pattern
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

Closures can use the same pattern matching behavior in parameters.

---

## 🔍 7. Refutability: Whether a Pattern Might Fail to Match

Patterns can be:

* **Irrefutable**: always match (e.g., `x` in `let x = 5;`)
* **Refutable**: might not match (e.g., `Some(x)` in `if let Some(x) = a_value`)

### Allowed Use Cases

* `let`, `for`, and function parameters: **only accept irrefutable** patterns
* `if let`, `while let`, and `let...else`: accept both, but warn on irrefutable patterns

### Invalid Example (Refutable Pattern in `let`)

```rust
let Some(x) = some_option_value; // Error!
```

Compiler output:

```
error[E0005]: refutable pattern in local binding
 --> src/main.rs:3:9
  |
3 |     let Some(x) = some_option_value;
  |         ^^^^^^^ pattern `None` not covered
```

### Fix Using `let...else`

```rust
let Some(x) = some_option_value else {
    return;
};
```

### Warning for Irrefutable Pattern in `let...else`

```rust
let x = 5 else {
    return;
};
```

Warning output:

```
warning: irrefutable `let...else` pattern
note: this pattern will always match, so the `else` clause is useless
```

---

## 📉 Pattern Syntax

Here we explore all valid pattern syntax in Rust:

### 🔢 Matching Literals

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

### 🤹 Matching Named Variables

```rust
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {y}"), // shadows outer y
    _ => println!("Default case, x = {x:?}"),
}

println!("at the end: x = {x:?}, y = {y}");
```

### ⬇️ Multiple Patterns with `|`

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

### ↔️ Ranges with `..=`

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

Also works with `char`:

```rust
let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

### 🧐 Destructuring Structs

```rust
struct Point { x: i32, y: i32 }

let p = Point { x: 0, y: 7 };
let Point { x, y } = p;
```

Or match with literals:

```rust
match p {
    Point { x, y: 0 } => println!("On x-axis at {x}"),
    Point { x: 0, y } => println!("On y-axis at {y}"),
    Point { x, y } => println!("On neither axis: ({x}, {y})"),
}
```

### ⚛️ Destructuring Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::ChangeColor(0, 160, 255);

match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move: {x}, {y}"),
    Message::Write(text) => println!("Text: {text}"),
    Message::ChangeColor(r, g, b) => println!("RGB: {r}, {g}, {b}"),
}
```

### 🌐 Nested Structs and Enums

```rust
enum Color { Rgb(i32, i32, i32), Hsv(i32, i32, i32) }

enum Message {
    ChangeColor(Color),
    // other variants
}

let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

match msg {
    Message::ChangeColor(Color::Rgb(r, g, b)) => println!("RGB: {r}, {g}, {b}"),
    Message::ChangeColor(Color::Hsv(h, s, v)) => println!("HSV: {h}, {s}, {v}"),
    _ => (),
}
```

### 🤖 Ignoring Values

```rust
fn foo(_: i32, y: i32) {
    println!("Only y is used: {y}");
}

let (first, _, third, _, fifth) = (2, 4, 8, 16, 32);
```

Use `_x` to suppress warnings but still bind:

```rust
let _x = 5; // no warning
let y = 10; // warning: unused variable
```

Using `_` vs `_s`:

```rust
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{s:?}"); // works: s not moved
```

### … `..` to Ignore Rest

```rust
struct Point { x: i32, y: i32, z: i32 }

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {x}"),
}
```

Or in tuples:

```rust
let numbers = (2, 4, 8, 16, 32);
match numbers {
    (first, .., last) => println!("{first}, {last}"),
}
```

Only one `..` allowed:

```rust
// Error: (.., second, ..) is ambiguous
```

### ⚠️ Match Guards with `if`

```rust
let num = Some(4);

match num {
    Some(x) if x % 2 == 0 => println!("Even: {x}"),
    Some(x) => println!("Odd: {x}"),
    None => (),
}
```

Used to avoid variable shadowing:

```rust
let x = Some(5);
let y = 10;

match x {
    Some(n) if n == y => println!("n = y = {n}"),
    _ => println!("x = {x:?}"),
}
```

Match guard applies to whole `|` group:

```rust
match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```

### 🎫 `@` Bindings

```rust
enum Message { Hello { id: i32 } }

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3..=7 } => println!("id in range: {id_variable}"),
    Message::Hello { id: 10..=12 } => println!("another range"),
    Message::Hello { id } => println!("other id: {id}"),
}
```

---

## 🌟 Summary

Rust’s pattern syntax allows destructuring, matching, and concise conditional control. Patterns appear in nearly every construct, and understanding them helps you write expressive, safe, and powerful Rust code.

---
