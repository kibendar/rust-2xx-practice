/// https://practice.course.rs/functional-programing/closure.html

#[test]
fn test204() {
    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    print();
    print();

    println!("{}",color);
}

#[test]
fn test205() {
    let mut count = 0;

    let mut inc = move || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();


    let _reborrow = &count;

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    assert_eq!(count, 0);
}

#[test]
fn test206() {
    // A non-copy type.
    let movable = Box::new(3);

    // A copy type would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        take(movable);
    };

    consume();
    // consume();
}

fn take<T>(_v: T) {

}

#[test]
fn test207() {
    // A non-copy type.
    let movable = Box::new(3);

    // A copy type would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        take1(&movable);
    };

    consume();
    consume();
}

fn take1<T>(_v: &T) {

}

#[test]
fn test208() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    /* Make it work, only changeg the following line */
    let n = example_closure(String::from("5"));
}

fn fn_once<F>(func: F)
where
    F: Fn(usize) -> bool,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

#[test]
fn test209() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}

#[test]
fn test210() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}",s);
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F)  {
    f("hello")
}

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
// The closure takes no input and returns nothing.
    F: FnOnce() {

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
// The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

#[test]
fn test211() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

#[test]
fn test212() {
    let mut s = String::new();

    let update_string = |str| -> String {s.push_str(str); s };

    exec1(update_string);
}

fn exec1<'a, F: FnOnce(&'a str) -> String>(mut f: F) {
    f("hello");
}

// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

#[test]
fn test213() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}

/* Fill in the blank and fix the error */
// You can also use `impl FnOnce(i32) -> i32`
fn create_fn() -> impl Fn(i32) -> i32 {
    let num = 5;

    move |x| x + num
}


#[test]
fn test214() {
    let fn_plain = create_fn();
    fn_plain(1);
}

// Every closure has its own type. Even if one closure has the same representation as another, their types are different.
fn factory(x:i32) -> Box<dyn Fn(i32) -> i32> {

    let num = 5;

    if x > 1{
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x + num)
    }
}

#[test]
fn test215() {}