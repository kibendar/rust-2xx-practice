fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut frequency = vec![0; 5]; // A frequency array for bird types 1 to 5.

    // Count the frequency of each bird type
    for &bird in &arr {
        frequency[(bird - 1) as usize] += 1;
    }

    // Find the bird type with the maximum frequency and the smallest id in case of a tie
    let mut max_count = 0;
    let mut bird_type = 0;

    for (i, &count) in frequency.iter().enumerate() {
        if count > max_count {
            max_count = count;
            bird_type = i + 1;
        }
    }

    bird_type as i32
}

#[test]
fn main() {
    let n = 6;
    let arr = vec![1, 4, 4, 5, 3, 4]; // Bird sightings

    let result = migratory_birds(arr);
    println!("{}", result);
}
