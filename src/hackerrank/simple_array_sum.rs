#[test]
fn main() {

    let array = [1, 2, 3];

    let mut sum: i32 = 0;

    for i in 0.. array.len() {
        sum += array[i];
    }

    print!("Sum of elements of array: {}", sum);
}