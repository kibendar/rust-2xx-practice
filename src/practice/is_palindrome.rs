fn is_palindrome(number: i32) -> bool {
    let num_str = number.to_string();
    let reversed_str: String = num_str.chars().rev().collect();
    num_str == reversed_str
}

#[test]
fn test() {
    let data =
        [
            (123, false),
            (121, true),
            (1221, true),
        ];


    data
        .iter()
        .for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
}

