fn count_permutation(shipments: &Vec<u32>) -> i32 {
    let n = shipments.len();
    let total_weight: u32 = shipments.iter().sum();

    // Check if the weight can be equally distributed
    if total_weight as usize % n != 0 {
        return -1;
    }

    let target_weight = (total_weight / n as u32) as i32;
    let mut moves = 0;
    let mut balance = 0;

    for &weight in shipments {
        balance += weight as i32 - target_weight;
        moves += balance.abs();
    }

    moves
}

#[test]
fn main() {
    let shipments = vec![1, 1, 1, 1, 6];
    let result = count_permutation(&shipments);
    println!("Minimum moves: {}", result);
}
