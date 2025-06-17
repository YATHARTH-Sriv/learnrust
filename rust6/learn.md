# 🧠 Things Learned In Chapter 6: Enums and Pattern Matching

---

## 📦 What Are Enums?

Enums give you a way of saying a value is **one of a possible set of values**.

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

---

## 💡 Embedding Data in Enum Variants

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

You can put **any kind of data** inside an enum variant: strings, numbers, or even structs!

```rust
struct Ipv4Addr {
    // ...
}

struct Ipv6Addr {
    // ...
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

---

## 🔀 Enums vs Structs

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32);

let m = Message::Write(String::from("hello"));
```

✅ Enums let you represent all these variants under **one type**.

---

## 🧩 Implementing Methods on Enums

```rust
impl Message {
    fn call(&self) {
        // method body here
    }
}
```

---

## 🎲 The `Option` Enum

Encodes a common scenario: **a value may be present or not**.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

🔒 `Option<T>` and `T` are different types — you **must unwrap** before using.

---

## 🧰 Methods on Option

| Method         | Description                                |
| -------------- | ------------------------------------------ |
| `unwrap()`     | Panic if `None`                            |
| `unwrap_or(x)` | Use `x` if `None`                          |
| `is_some()`    | Check if `Some`                            |
| `is_none()`    | Check if `None`                            |
| `map()`        | Transform the value if `Some`              |
| `and_then()`   | Chain another operation returning `Option` |
| `match`        | Full control pattern matching              |

---

## 🎭 Pattern Matching with Enums

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

---

## 🔒 Exhaustive Pattern Matching

Rust’s `match` forces you to handle **all possible cases**:

```rust
let dice_roll = 9;

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}
```

| Concept            | Example        | Meaning                                   |
| ------------------ | -------------- | ----------------------------------------- |
| Catch-all variable | `other => ...` | Catches unmatched value and binds it      |
| Wildcard `_`       | `_ => ...`     | Catches unmatched value *without* binding |
| Unit `()`          | `_ => ()`      | Catch-all case, do nothing                |
| Match order        | Top to bottom  | Earlier arms take priority                |

---

## ✨ Cleaner Matching with `if let` & `let...else`

### `if let`

```rust
if let Some(x) = maybe_value {
    println!("Got {x}");
} else {
    println!("None!");
}
```

### `let...else`

```rust
let Some(x) = maybe_value else {
    return;
};

println!("Got {x}");
```

✅ `let...else` helps keep your logic on the **"happy path"** and avoids nesting.

---

