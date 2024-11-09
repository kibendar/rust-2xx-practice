fn birthday(s: Vec<i32>, d: i32, m: i32) -> i32 {
    let n = s.len();
    let mut count = 0;

    // Loop through the chocolate bar
    for i in 0..=n - m as usize {
        // Calculate the sum of the segment starting at index i with length m
        let sum: i32 = s[i..i + m as usize].iter().sum();

        // Check if the sum matches the birth day d
        if sum == d {
            count += 1;
        }
    }

    count
}

#[test]
fn main() {
    // Input
    let n = 5; // number of squares
    let s = vec![1, 2, 1, 3, 2]; // the numbers on each square
    let d = 3; // Ron's birth day
    let m = 2; // Ron's birth month

    // Output the result
    let result = birthday(s, d, m);
    println!("{}", result); // Should print 2
}
