/// https://practice.course.rs/lifetime/static.html

#[test]
fn test194() {
    let v: &str = "hello";
    need_static(v);

    println!("Success!")
}

fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}

// #[derive(Debug)]
// struct Config {
//     a: String,
//     b: String,
// }
//
//
// fn init() -> Option<&'static mut Config> {
//     let c = Box::new(Config {
//         a: "A".to_string(),
//         b: "B".to_string(),
//     });
//
//     Some(Box::leak(c))
// }
//
//
// fn test195() {
//     unsafe {
//         config = init();
//
//         println!("{:?}",config)
//     }
// }

#[test]
fn test196() {
    // Make a `string` literal and print it:
    let static_string = "I'm in read-only memory";
    println!("static_string: {}", static_string);

    println!("static_string reference remains alive: {}", static_string);
}

use std::fmt::Debug;

fn print_it<T: Debug + 'static>( input: T) {
    println!( "'static value passed in is: {:?}", input );
}

fn print_it1( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}


fn print_it2<T: Debug + 'static>( input: &T) {
    println!( "'static value passed in is: {:?}", input );
}

#[test]
fn test197() {
    // i is owned and contains no references, thus it's 'static:
    const i:i32 = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);

    print_it1(&i);

    // but this one WORKS !
    print_it2(&i);
}