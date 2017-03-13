use std::io;

fn main() {
    println!("Factorial Calculator");

    println!("Enter a number");

    let mut num = String::new();

    io::stdin().read_line(&mut num)
        .expect("Failed to read line");

    let num: u64 = num.trim().parse()
        .expect("Please Enter a number");

    println!("Factorial of {} is {}", num, factorial(num));
}

fn factorial(x: u64) -> u64 {
    if x <= 1 {
        return 1;
    } else {
        return x * factorial(x-1);
    }
}
