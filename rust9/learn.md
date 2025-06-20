# 🧠 Things Learned In Chapter 9: Error Handling

## Unrecoverable Errors : panic!

        example #1

        panic!("intentional panic here ")

        example #2

        let v=vec![1,2,3]

        v[100]  ----->> this causes panic

# 🚨 What Happens When a Panic Occurs?
By default, when your Rust program panics, it does the following:


1. Unwinds the Stack

This means Rust goes backward through the function call stack (like rewinding).

For each function, it:

Runs the destructors (drop) for all local variables.

Frees heap memory allocated by that function.

This is called "stack unwinding", and it allows your program to clean up safely.

🔁 Analogy: Think of it as carefully unstacking a pile of dishes, one at a time, cleaning each dish as you go.

---
---

# 🧨 Alternative: Abort Immediately
Sometimes, you don’t care about cleaning up. For example:

You want a smaller binary (e.g., in embedded systems or WebAssembly).

You don’t want the extra performance cost of cleanup.

You know the program is beyond saving anyway.

In that case, Rust lets you tell the compiler:

"Don’t unwind; just abort the program right away."

This is called aborting on panic.

## 🛠 How to Enable Abort on Panic

In your Cargo.toml, under the [profile] section, you can configure it like this:


    [profile.release]
    panic = 'abort'

    This will apply only in release mode (i.e., when you build with cargo build --release).

If you want to do it in debug mode, add:


    [profile.dev]
    panic = 'abort'

####  buffer overread
    
    In C if you do this v[100] it can show you whatever is at the location in memory 

    Safety Vulnerability

# 🧠 What is a Backtrace?
A backtrace is a list of all the function calls that led up to a panic or error in your program.

Think of it like a breadcrumb trail that shows exactly how your program got to the point where it crashed. It helps you debug by showing you the path your code took, step by step.

    How To Do IT

    RUST_BACKTRACE=1 cargo run

    Reading A Backtrace 

    thread 'main' panicked at src/main.rs:4:6:
    index out of bounds: the len is 3 but the index is 99
    stack backtrace:
    0: rust_begin_unwind
    1: core::panicking::panic_fmt
    2: core::panicking::panic_bounds_check
    3: slice::index::SliceIndex::index
    4: panic::main          ← 🔍 This is your code!


# .unwrap()

    If Ok enum value is returned it give value otherwise calls the panic

## .expect()

    expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier

    use std::fs::File;

    fn main() {
        let greeting_file = File::open("hello.txt")
            .expect("hello.txt should be included in this project");
    }

## Uisng The ? opearot for better error handling:

        use std::fs::File;
        use std::io::{self, Read};

        fn read_username_from_file() -> Result<String, io::Error> {
            let mut username = String::new();

            File::open("hello.txt")?.read_to_string(&mut username)?;

            Ok(username)
        }

----

        ? uses the From trait which converts any error into our Myerror type as defined in the return type of a function

        This error points out that we’re only allowed to use the ? operator in a function that returns Result, Option

#### Something new: 

        use std::error::Error;
        use std::fs::File;

        fn main() -> Result<(), Box<dyn Error>> {
            let greeting_file = File::open("hello.txt")?;

            Ok(())
        }

        As we know for using ? we have to return a Result<"""" the type of data going to be returned"""",error> or Option<T>
        so in this way we can use ? in our main funciton as well 

        Box<dyn Error> is  

    🔍 Let's break it down
    
    1. dyn Error — A Trait Object
    Error is a trait (std::error::Error) that all error types implement (like io::Error, fmt::Error, etc.)

    dyn Error means any type that implements the Error trait

    You’re using dynamic dispatch — the actual error type is determined at runtime

    2. Box<dyn Error> — Boxing it
    You can't use trait objects directly on the stack because they don’t have a fixed size

    So you put them on the heap using Box<>

    Think of Box<dyn Error> as:

    "I don’t care what kind of error it is, just give me a box that contains some error."
