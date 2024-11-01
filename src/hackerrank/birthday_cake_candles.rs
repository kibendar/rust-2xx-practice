use std::io;
use std::io::BufRead;

fn birthday_cake_candles(candles: Vec<u32>) -> u32 {
    let max_height = candles.iter().max().unwrap();
    candles.iter().filter(|&&height| height == *max_height).count() as u32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line (number of candles)
    let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Read the second line (heights of the candles)
    let candles: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Print the result
    println!("{}", birthday_cake_candles(candles));
}