use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn a_very_big_sum(ar: &[i64]) -> i64 {
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Create output file based on the environment variable "OUTPUT_PATH"
    let mut fptr = File::create(env::var("C:\\rust-2xx-practice\\src\\hackerrank\\output.txt").unwrap()).unwrap();

    // Read the number of elements (though we don't use it)
    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the array of integers
    let ar: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // Calculate the sum using the function
    let result = a_very_big_sum(&ar);

    // Write the result to the output file
    writeln!(&mut fptr, "{}", result).ok();
}
