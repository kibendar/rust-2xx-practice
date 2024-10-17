/// https://practice.course.rs/ownership/ownership.html
fn test46() {
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}
fn test47() {
    let x = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}
fn test48() {
    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}
fn test49() {
    let x = String::from("hello, world");
    let y = x.as_str();
    println!("{},{}",x,y);
}
// Don't modify code in main!
fn test50() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
fn test51() {
    let s = give_ownership2();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership2() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    let _s = s.as_bytes();
    s
}
fn test52() {
    let s = give_ownership2();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership3() -> String {
    let s = String::from("hello, world");
    s
}
fn test53() {
    let s = String::from("hello, world");

    print_str1(s.clone());

    println!("{}", s);
}

fn print_str1(s: String)  {
    println!("{}",s)
}
fn test54() {
    let s = String::from("hello, world");
    print_str2(&s);
    println!("{}", s);
}
fn print_str2(s: &String)  {
    println!("{}",s)
}
fn test55() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}
fn test56() {
    let s = String::from("hello, ");

    // modify this line only !
    let mut s1 = s;

    s1.push_str("world")
}
fn test57() {
    let x = Box::new(5);

    let mut y = Box::new(3);       // implement this line, dont change other lines!

    *y = 4;

    assert_eq!(*x, 5);
}
fn test58() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // modify this line only, don't use `_s`
    println!("{:?}", t.1);
}
fn test59() {
    let t = (String::from("hello"), String::from("world"));

    // fill the blanks
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}