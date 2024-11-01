fn plus_minus(arr: &[i32]) {
    let n = arr.len() as f64;
    let (mut positive_count, mut negative_count, mut zero_count) = (0, 0, 0);

    for &num in arr {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    println!("{:.6}", positive_count as f64 / n);
    println!("{:.6}", negative_count as f64 / n);
    println!("{:.6}", zero_count as f64 / n);
}

#[test]
fn main() {
    let arr = vec![-4, 3, -9, 0, 4, 1];
    plus_minus(&arr);
}
