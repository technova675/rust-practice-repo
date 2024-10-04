use std::io;

fn main() {
    println!("Enter the number of terms:");

    // Reading user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Converting input to an integer
    let terms: u32 = input.trim().parse().expect("Please enter a valid number");

    // Printing Fibonacci series
    println!("Fibonacci Series:");
    fibonacci_series(terms);
}

fn fibonacci_series(terms: u32) {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..terms {
        println!("{}", a);
        let temp = a;
        a = b;
        b = temp + b;
    }
}
