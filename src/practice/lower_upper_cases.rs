
#[test]
fn test() {
    let data =
        [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];


    data
        .iter()
        .for_each(|(a, b)| {
            assert_eq!(
                invert_the_case(a.to_string()),
                b.to_string()
            );
            assert_eq!(
                invert_the_case(b.to_string()),
                a.to_string()
            );
        });
}
fn invert_the_case (s: String) -> String {
    s.chars()
        .map(|c|{
            if c.is_lowercase(){
                c.to_uppercase().to_string()
            }else {
                c.to_lowercase().to_string()
            }
        })
        .collect()
}
