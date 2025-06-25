# 🧠 Things Learned In Chapter 13: Iterators Closures in Rust

---

### 🔹 What Are Closures?

Closures are **anonymous functions** that can capture variables from their surrounding environment.

* You can:

  * Store them in variables
  * Pass them as arguments to functions
  * Return them from functions

```rust
let add = |a, b| a + b;
println!("{}", add(2, 3)); // 5
```

---

### 🔹 Capturing the Environment

Closures **automatically capture** variables from the scope where they're defined. There are 3 ways a closure can capture values:

1. **By reference** (`&T`)
2. **By mutable reference** (`&mut T`)
3. **By value** (`T`)

Rust infers how to capture values based on how the closure uses them.

---

### 🔹 Example: Shirt Giveaway

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue { ShirtColor::Red } else { ShirtColor::Blue }
    }
}
```

* `unwrap_or_else` takes a **closure** that runs **only if** the option is `None`.
* The closure captures `self` (the `Inventory` instance).

---

### 🔹 Fn Traits Overview

Closures automatically implement one or more of the following traits:

| Trait    | Behavior                          | Use Case                 |
| -------- | --------------------------------- | ------------------------ |
| `FnOnce` | Can be called once (moves values) | Consumes captured values |
| `FnMut`  | Can mutate captured values        | Mutable environment      |
| `Fn`     | Reads or captures nothing         | Read-only or no capture  |

**Hierarchy**:

```text
Fn ⊆ FnMut ⊆ FnOnce
```

---

### 🔹 unwrap\_or\_else Example

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

* The closure is only called if the option is `None`.
* Requires only `FnOnce` because it is called at most one time.

---

### 🔹 Using Function Names Instead of Closures

If a closure doesn't capture anything:

```rust
let opt: Option<Vec<i32>> = None;
let v = opt.unwrap_or_else(Vec::new);
```

* This is equivalent and preferred if the function doesn't capture any environment.

---

### 🔹 sort\_by\_key and FnMut

```rust
list.sort_by_key(|r| r.width);
```

* This closure captures nothing, so it can implement all Fn traits.
* `sort_by_key` requires `FnMut` because it calls the closure **multiple times**.

---

### 🔹 FnOnce Error Example

```rust
let value = String::from("moved value");
list.sort_by_key(|r| {
    sort_operations.push(value); // ❌ moved here
    r.width
});
```

* Error: cannot move out of captured variable in `FnMut` closure.
* This closure only implements `FnOnce`, so it's not compatible with `sort_by_key`.

---

### ✅ Fix Using FnMut

```rust
let mut num_sort_operations = 0;
list.sort_by_key(|r| {
    num_sort_operations += 1;
    r.width
});
```

* Mutably captures `num_sort_operations`.
* This is valid because it satisfies `FnMut`.

---


### 🔹 What is an Iterator?

* An **iterator** handles logic to loop over elements and determine when the sequence ends.

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
```

* This alone does **nothing** until the iterator is used:

```rust
for val in v1_iter {
    println!("Got: {val}");
}
```

---

### 🔹 The `Iterator` Trait and `.next()` Method

* Iterators implement the `Iterator` trait which requires a `.next()` method:

```rust
fn iterator_demo() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

* Note: `.next()` mutates the iterator internally. That’s why it needs to be `mut`.
* In a `for` loop, the iterator is made mutable automatically.

---

### 🔹 `.iter()` vs `.into_iter()` vs `.iter_mut()`

| Method         | What It Yields         | Description                        |
| -------------- | ---------------------- | ---------------------------------- |
| `.iter()`      | `&T` (reference)       | Iterates over immutable references |
| `.into_iter()` | `T` (owned values)     | Takes ownership of the collection  |
| `.iter_mut()`  | `&mut T` (mutable ref) | Iterates over mutable references   |

---

### 🔹 `.sum()` Consumes Iterator

* `.sum()` is a **consuming adapter** — it takes ownership of the iterator:

```rust
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}
```

---

### 🔹 Iterator Adapters: `map`, `filter`, etc.

#### 🔸 `.map()`

* Transforms each item.
* **Lazily evaluated** — must call `.collect()` to execute.

```rust
let v1 = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2, 3, 4]);
```

#### 🔸 `.filter()`

* Takes a **closure** returning `bool`.
* Keeps items where closure returns `true`.

```rust
let v1 = vec![1, 2, 3, 4];
let evens: Vec<_> = v1.into_iter().filter(|x| x % 2 == 0).collect();
assert_eq!(evens, vec![2, 4]);
```

---

### 🔹 Lazy Evaluation and Chaining

* Most iterator methods are **lazy**.
* You can **chain** `.map()`, `.filter()`, etc., and execute with `.collect()`, `.sum()`, etc.

```rust
let result: Vec<_> = (1..10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 10)
    .collect();
```

---

## Iterators:

         An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished

             let v1 = vec![1, 2, 3];

             let v1_iter = v1.iter();

             Now it does not do anything on its own 

                    let v1 = vec![1, 2, 3];

                    let v1_iter = v1.iter();

                    for val in v1_iter {
                        println!("Got: {val}");
                    }
             
 ### The Iterator Trait and the next Method

             .next()

              the next method, which returns one item of the iterator at a time, wrapped in Some and, when iteration is over, returns None. 

               fn iterator_demonstration() {
                let v1 = vec![1, 2, 3];

                let mut v1_iter = v1.iter();

                assert_eq!(v1_iter.next(), Some(&1));
                assert_eq!(v1_iter.next(), Some(&2));
                assert_eq!(v1_iter.next(), Some(&3));
                assert_eq!(v1_iter.next(), None);
                } 

           Note that we needed to make v1_iter mutable: calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence. In other words, this code consumes, or uses up, the iterator. Each call to next eats up an item from the iterator. We didn’t need to make v1_iter mutable when we used a for loop because the loop took ownership of v1_iter and made it mutable behind the scenes.

## Differcne between .iter  and .into_iter and .iter_mut:

    iter method produces an iterator over immutable references

     ownership of v1 and returns owned values, we can call into_iter
    
     if we want to iterate over mutable references, we can call iter_mut


# sum on iterators:

    sum method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator

     fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

## Methods that Produce Other Iterators ( map, filter):

    .map()

        let v1: Vec<i32> = vec![1, 2, 3];

        v1.iter().map(|x| x + 1);

        Does Not Do Anything 
        add .collect()

            let v1: Vec<i32> = vec![1, 2, 3];

            let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

            assert_eq!(v2, vec![2, 3, 4]);

        Use When To DO operations

    .filter() :

            Many iterator adapters take closures as arguments 

             filter method that takes a closure. The closure gets an item from the iterator and returns a bool. If the closure returns true, the value will be included in the iteration produced by filter. If the closure returns false, the value won’t be included.


    For eg go to lib.rs with tests

    You can chain multiple calls to iterator adapters to perform complex actions in a readable way. But because all iterators are lazy, you have to call one of the consuming adapter methods to get results from calls to iterator adapters.
