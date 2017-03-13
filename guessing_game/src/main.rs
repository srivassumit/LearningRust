extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess The Number");

    let secret_number = rand::thread_rng().gen_range(1,101);

    //println!("The secret number is:{}", secret_number);
    loop {
        println!("Please input your Guess");

    // let keyword is used to assign variables.
    // rust variables are immutable by default.
    // the mut keyword tells that this variable is mutable
        let mut guess = String::new();// new creates a new string.
    // the :: thing means that new is like a static method of String class.

    // read_line takes a reference to a mutable String as argument.
    // It then assigns the value from Std in to the mutable String reference passed.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    // read_line returns a io:Result which encodes error handling info.
    // expect is a method of io::Result
    // on error, the program stops and displays the message given in expect method.
    //not giving expect wil give a warning at compile time.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {println!("You Win!");break;}
        }
    }
}
