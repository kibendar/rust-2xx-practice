fn apple_and_orange(s: i32, t: i32, a: i32, b: i32, apples: Vec<i32>, oranges: Vec<i32>) {
    // Count apples that fall within the range of the house
    let apple_count = apples.iter()
        .map(|&distance| a + distance) // Calculate the landing position of each apple
        .filter(|&position| position >= s && position <= t) // Check if it falls on the house
        .count();

    // Count oranges that fall within the range of the house
    let orange_count = oranges.iter()
        .map(|&distance| b + distance) // Calculate the landing position of each orange
        .filter(|&position| position >= s && position <= t) // Check if it falls on the house
        .count();

    // Print the results
    println!("{}", apple_count);
    println!("{}", orange_count);
}
#[test]
fn main() {
    // Example input
    let s = 7;
    let t = 11;
    let a = 5;
    let b = 15;
    let apples = vec![-2, 2, 1];
    let oranges = vec![5, -6];

    // Call the function with the example input
    apple_and_orange(s, t, a, b, apples, oranges);
}
