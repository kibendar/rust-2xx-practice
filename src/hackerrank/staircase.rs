use std::io;

fn staircase(n: usize) {
    for i in 1..=n {
        // Print spaces followed by '#' characters
        println!("{:>width$}", "#".repeat(i), width = n);
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid number");

    staircase(n);
}
