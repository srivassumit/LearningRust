use std::io;

fn main() {
    println!("Guess The Number");

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

    println!("You guessed: {}", guess);
}
