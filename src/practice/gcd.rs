fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[test]
fn main() {
    let num1 = 48;
    let num2 = 18;
    let result = gcd(num1, num2);
    println!("The GCD of {} and {} is {}", num1, num2, result);
}
