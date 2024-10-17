/// https://practice.course.rs/variables.html
#[test]
fn test1() {

    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");

}
#[test]
fn test2() {
    let mut x: i32 = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

#[test]
fn test3() {
    let x: i32 = 10;
    let y: i32 = 5;
    {

        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}

#[test]
fn test4() {
    fn main() {
        let x = define_x();
        println!("{}, world", x);
    }

    fn define_x() -> String {
        let x = String::from("hello");
        x
    }

}

#[test]
fn test5() {
    // Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)

    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

#[test]
fn test6() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    // let x = x;   I remove this line of coded!!!!!!!!!!!
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}

#[test]
fn test7() {
    let (mut x, y) = (1, 2);  // Edited (mut x, y)
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

#[test]
fn test8() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success!");
}