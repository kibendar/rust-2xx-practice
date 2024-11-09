fn bonAppetit(bill: Vec<i32>, k: usize, b: i32) {
    // Calculate the total cost excluding the item Anna didn't eat
    let total: i32 = bill.iter().sum();
    let anna_share = (total - bill[k]) / 2;

    // Check if Brian overcharged Anna
    if b == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - anna_share);
    }
}

#[test]
fn main() {
    // Sample input
    let n = 4;
    let k = 1;
    let bill = vec![3, 10, 2, 9];
    let b = 12;

    // Call the function to check if the calculation is correct
    bonAppetit(bill, k, b);
}
