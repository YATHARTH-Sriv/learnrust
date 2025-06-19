use crate::hello::greet;

use hello::hosting::greet_from_hosting_child_of_hello;

mod hello;

fn main() {
    println!("Hello, world!");
    greet();
    greet_from_hosting_child_of_hello();
}
