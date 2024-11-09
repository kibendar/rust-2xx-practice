fn breaking_records(scores: Vec<i32>) -> [i32; 2] {
    let mut max_record = scores[0];
    let mut min_record = scores[0];
    let mut max_break_count = 0;
    let mut min_break_count = 0;

    for &score in &scores[1..] {
        if score > max_record {
            max_record = score;
            max_break_count += 1;
        }
        if score < min_record {
            min_record = score;
            min_break_count += 1;
        }
    }

    [max_break_count, min_break_count]
}

#[test]
fn main() {
    let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
    let result = breaking_records(scores);
    println!("{:?}", result); // Output: [2, 4]
}
