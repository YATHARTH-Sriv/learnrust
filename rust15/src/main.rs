#[derive(Debug)]
enum List{
    Cons(i64,Box<List>),
    Nil
}

use std::ops::Deref;




fn main() {
    println!("Hello, world!");

    let list=List::Cons(3, Box::new(List::Cons(3, Box::new(List::Nil))));

    println!("{:?}",list);
}


