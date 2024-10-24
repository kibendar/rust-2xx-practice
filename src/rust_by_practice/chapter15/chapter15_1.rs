/// https://practice.course.rs/formatted-output/println.html
#[test]
fn test169() {
    let s1 = "hello";
    /* Fill in the blank */
    let s = format!("{}, world!", s1);
    assert_eq!(s, "hello, world!");
}
#[test]
fn test170() {
    print!("hello world, ");
    println!("I am");
    println!("Sunface!");
}