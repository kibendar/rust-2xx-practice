fn divisible_sum_pairs(n: usize, k: i32, ar: Vec<i32>) -> i32 {
    let mut count = 0;

    // Loop over all pairs (i, j) where i < j
    for i in 0..n {
        for j in i+1..n {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

#[test]
fn main() {
    // Input
    let n = 6;
    let k = 3;
    let ar = vec![1, 3, 2, 6, 1, 2];

    // Output the result
    let result = divisible_sum_pairs(n, k, ar);
    println!("{}", result); // Should print 5
}
