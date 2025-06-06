use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Welcome To Guessing Game");

    let random_num= rand::random_range(0..100);
    println!("random number {}",random_num);

    loop{

    println!("Guess The Number");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // let guess: u32 = guess.trim().parse().expect("Please type a number!");
    let guess: u32 = match guess.trim().parse() {
        Ok(num)=>num,
        Err(_)=>continue,
    };

    match guess.cmp(&random_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break
        },
    }

    println!("You guessed: {}", guess);

    }
    println!("Game Ends");
}
