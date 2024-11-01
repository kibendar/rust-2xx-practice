fn mini_max_sum(arr: &[u64; 5]) {
    let total_sum: u64 = arr.iter().sum();
    let min_sum = total_sum - arr.iter().max().unwrap();
    let max_sum = total_sum - arr.iter().min().unwrap();
    println!("{} {}", min_sum, max_sum);
}

#[test]
fn main() {
    let arr = [1, 3, 5, 7, 9];
    mini_max_sum(&arr);
}
