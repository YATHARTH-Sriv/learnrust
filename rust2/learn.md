# Things Learned In this first chapter:

## Prelude

    basically it bring all the imports of library to be accesible
    eg: use tokio::prelude::*


## Refernce 

    Rust uses references (&) to allow one piece of code to access data without taking ownership.


## mutability

     let mut x = 5;
     x = 6; // ✅ Works!


## Binary and library
  Rust projects can be either:

        📦 Binary Crate
        Contains a main() function.

        Compiles to an executable.

        Created with: cargo new my_app


        📚 Library Crate
        No main() function.

        Intended for reuse across projects.

        Created with: cargo new my_utils --lib


## For Every Create or dependicney docs

    cargo doc --open

## Shadowing

    Shadowing means you can declare a new variable with the same name as an existing one using let.

    This new variable shadows (hides) the old one, effectively replacing it in that scope.