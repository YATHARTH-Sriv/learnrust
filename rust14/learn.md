# 🧠 Things Learned In Chapter 14: Cargo and Crates Rust

    cargo build

    cargo build --release

    opt-level ----> the amount optimizations Rust will apply to your code

    [profile.dev]
    opt-level = 0

    [profile.release]
    opt-level = 3

## Documentation For Your Crate:

    Write Docs As markdown files by /// 

    See examp,e in lib.rs file 

    then
        cargo doc ----->> Generated the Docs HTML page

### Commonly Used Sections

    The # Examples Markdown heading creates a section in the HTML with the title “Examples.”

        Here are some other sections that crate authors commonly use in their documentation:

            1. Panics: The scenarios in which the function being documented could panic. 
            
            2. Errors: If the function returns a Result, describing the kinds of errors that might occur and what conditions might cause those errors to be returned 

            3. Safety: If the function is unsafe to call (we discuss unsafety in Chapter 20), there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold.
            

## Documention Tests:

    cargo test will run the code examples in your documentation as tests