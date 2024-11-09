use std::cmp::Ordering;

// Function to calculate the GCD of two numbers
fn gcd(x: i32, y: i32) -> i32 {
    match y.cmp(&0) {
        Ordering::Equal => x,
        _ => gcd(y, x % y),
    }
}

// Function to calculate the LCM of two numbers
fn lcm(x: i32, y: i32) -> i32 {
    (x * y) / gcd(x, y)
}

// Function to calculate the LCM of an array of numbers
fn lcm_of_array(arr: &[i32]) -> i32 {
    arr.iter().fold(1, |acc, &num| lcm(acc, num))
}

// Function to calculate the GCD of an array of numbers
fn gcd_of_array(arr: &[i32]) -> i32 {
    arr.iter().fold(arr[0], |acc, &num| gcd(acc, num))
}

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let lcm_a = lcm_of_array(a);
    let gcd_b = gcd_of_array(b);
    let mut count = 0;

    // Check all multiples of LCM that are less than or equal to the GCD of array b
    let mut multiple = lcm_a;
    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

#[test]
fn main() {
    let a = vec![2, 6];
    let b = vec![24, 36];
    let result = get_total_x(&a, &b);
    println!("{}", result); // Should output 2
}
