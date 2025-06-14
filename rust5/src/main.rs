#[derive(Debug)]
struct User{
    name:String,
    password:String,
    loggedin:bool
}

use std::io;
fn main() {
    println!("Hello, world!");
    println!("Enter Details");


    let new_user=User{
        name:String::from("yatharth"),
        loggedin:false,
        password:String::from("hello")

    };
    println!("struct {new_user:?}");
    println!("struct secnd {:?}",new_user);
    println!("struct third {:#?}",new_user.name);

    let mut second_user=User{
       loggedin:false,
       name:String::from("second"),
       password:String::from("world")
    };

    second_user.name=String::from("changed to third");

    let third_user=User{
        name:String::from("third"),
        ..new_user
    };


    }
