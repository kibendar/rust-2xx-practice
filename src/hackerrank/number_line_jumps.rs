fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> &'static str {
    // Check if the velocities are different
    if v1 != v2 {
        // Calculate the difference in positions and velocities
        let position_diff = x2 - x1;
        let velocity_diff = v1 - v2;

        // Check if the position difference is divisible by the velocity difference
        // and if the division would result in a positive number of jumps
        if position_diff % velocity_diff == 0 && position_diff / velocity_diff >= 0 {
            return "YES";
        }
    } else if x1 == x2 {
        // If velocities are the same and starting points are the same
        return "YES";
    }
    "NO"
}

#[test]
fn main() {
    let (x1, v1, x2, v2) = (0, 3, 4, 2);
    let result = kangaroo(x1, v1, x2, v2);
    println!("{}", result);
}
