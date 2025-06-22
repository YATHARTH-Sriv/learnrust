# 🧠 Things Learned In Chapter 11: Tests in Rust

---

## ✨ What Are Tests in Rust?

Rust has built-in support for writing **unit tests** that verify whether your code behaves as expected.

* Test functions are marked with the `#[test]` attribute.

* Tests are executed using:

  ```
    cargo test
  ```

* Rust compiles your code in test mode and builds a test binary that runs each test **in parallel**.

---

## ✅ The `#[test]` Attribute

* Marks a function as a test case.
* Should be placed right above the test function.

Example:

```
#[test]
fn my_test() {
    assert!(true);
}
```

When running `cargo test`, it runs all functions annotated with `#[test]`.

---

## 🔎 Test Macros

### 1. `assert!`

* Takes a **boolean expression**.
* If `true`, test passes. If `false`, it `panic!`s and the test fails.

Example:

```
assert!(2 + 2 == 4);
```

---

### 2. `assert_eq!`

* Checks **equality**.
* Fails if `left != right`.

Example:

```
assert_eq!(add(2, 3), 5);
```

* Shows clear error message:

  ```
    assertion failed: `(left == right)`
  ```

---

### 3. `assert_ne!`

* Checks **non-equality**.
* Passes if values are different, fails if they are equal.

Example:

```
assert_ne!(2 * 2, 5);
```

---

## ✨ Debugging Assertion Failures

To compare **custom structs**, you need:

```
#[derive(PartialEq, Debug)]
```

* `PartialEq`: to use `==` and `!=`
* `Debug`: to display values in test failure output

Example:

```
#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

assert_eq!(rect1, rect2);
```

---

## 🖊️ Custom Error Messages

You can pass custom failure messages to `assert!`, `assert_eq!`, etc.

Example:

```
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

---

## 🧰 #\[cfg(test)]

Used to **conditionally compile** test code only when `cargo test` is run:

```
#[cfg(test)]
mod tests {
    // test functions here
}
```

This ensures that your tests are **not compiled into the final release binary**.

---

## 🔧 Organizing Tests

* You can write tests in the same file inside a `#[cfg(test)] mod tests {}`.
* You can also create a `tests/` directory and place integration tests there.
* Main code typically lives in `src/lib.rs` or `src/main.rs`, tests can call any `pub` function from those.

---

## ❓ Why Do Tests Sometimes Need `PartialEq`?

When you use `assert_eq!` or `assert_ne!` with custom types (like structs), Rust needs to know **how to compare them**.

* That's where the `PartialEq` trait comes in.
* Without it, Rust doesn’t know how to check `==` or `!=`.

Always pair it with `Debug` to see useful error messages.

---

## 🏆 Summary

* Use `#[test]` to write unit tests
* Use `assert!`, `assert_eq!`, and `assert_ne!` to verify correctness
* Derive `PartialEq` and `Debug` when asserting custom types
* Use `#[cfg(test)]` to isolate test code
* Run tests using:

  ```
    cargo test
  ```

Unit testing is an essential tool for safe, maintainable Rust code. Happy testing!

## Tests:
          Understand:

            cargo run compiles your code and then runs the resultant binary

            cargo test compiles your code in test mode and runs the resultant test binary. 
                - The default behavior of the binary produced by cargo test is to run all the tests in parallel 
                - capture output generated during test runs, 
                - preventing the output from being displayed and making it easier to read the output related to the test result
            
            the arguments that go to cargo test followed by the separator -- and then the ones that go to the test binary.

            the tests run in parallel so sometimes this can cause issue lets say multiple tests read same file it can cause issues:

             cargo test -- --test-threads=1

            The printed values are only shown for failed tests not for passed tests

            If You Want To See:

                cargo test -- --show-output
            
            Running particular tests not the whole test suite:

                cargo test one_hundred // only one possible not this cargo test one_hundred first_test no it does not work

            Filtering to Run Multiple Tests

                cargo test add // All the tests with name containng add will run

            Expensive tests 

                When You want some test to be ignored use #[ignore]

                eg:
                    #[cfg(test)]
                        mod tests {
                            use super::*;

                            #[test]
                            fn it_works() {
                                let result = add(2, 2);
                                assert_eq!(result, 4);
                            }

                            #[test]
                            #[ignore]
                            fn expensive_test() {
                                // code that takes an hour to run
                            }
                        }
                
                cargo test will run all test except with ignored trait

                to run ignored tests only
                    
                    cargo test -- --ignored 

                Run All Tests:

                    cargo test -- --include-ignored


## Types Of Tests:

    1. Unit Tests:

        - The purpose of unit tests is to test each unit of code in isolation
        - To pinpoint where code is and isn’t working as expected

    We have to name the module tests and use 
        #[cfg(test)] ---- > Tells compiler to run this module when cargo test not when cargo run or cargo build

        #[test] ----- > Tells compiler to run this functions and not the other helper functions

    Rust allows you to test private function    

    children tree can call the parent tree

    eg:
            pub fn add_two(a: usize) -> usize {
                internal_adder(a, 2)
            }

            fn internal_adder(left: usize, right: usize) -> usize {
                left + right
            }

            #[cfg(test)]
            mod tests {
                use super::*;

                #[test]
                fn internal() {
                    let result = internal_adder(2, 2);
                    assert_eq!(result, 4);
                }
            }
    

    2. Integration Tests:

          - They are External Tests
          - Use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API 
          - Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well



        To create integration tests, you first need a tests directory

         
         adder
            ├── Cargo.lock
            ├── Cargo.toml
            ├── src
            │   └── lib.rs
            └── tests
                └── integration_test.rs

        Can have multiple files in the tests directory
        Cargo will compile each of the files as an individual crate.

        Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.

          
           