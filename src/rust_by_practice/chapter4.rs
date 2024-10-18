/// https://practice.course.rs/basic-types/numbers.html
#[test]
fn test11() {
    let x: i32 = 5;
    let mut y: i32 = 5;

    y = x;

    let z = 10; // Type of z ?

    println!("Success!");
}

#[test]
fn test12() {
    let v: u16 = 38_u8 as u16;
    println!("Success!");
}

#[test]
fn test13() {
    fn main() {
        let x = 5;
        assert_eq!("i32", type_of(&x));

        println!("Success!");
    }


    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }

}

#[test]
fn test14() {
    fn main() {
        assert_eq!(i8::MAX, 127);
        assert_eq!(u8::MAX, 255);

        println!("Success!");
    }

}

#[test]
fn test15() {
    fn main() {
        let v1 = 251_u8.checked_add(8).unwrap();
        let v2 = i32::checked_add(251, 8).unwrap();

        println!("{}, {}", v1, v2);
    }

}

#[test]
fn test16() {
    fn main() {
        let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
        assert!(v == 1579);

        println!("Success!");
    }

}

#[test]
fn test17() {
    fn main() {
        let x = 1_000.000_1;
        let y: f32 = 0.12;
        let z = 0.01_f64;

        assert_eq!(type_of(&x), "f64".to_string());
        println!("Success!");
    }

    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }

}

#[test]
fn test18() {
    fn main() {
        assert!(0.1_f32+0.2_f32==0.3_f32);
    }
}
///Range
#[test]
fn test19() {
    fn main() {
        let mut sum = 0;
        for i in -3..2 {
            sum += i;
        }

        assert!(sum == -3);

        for c in 'a'..='z' {
            println!("{}", c);
        }
    }

}

#[test]
fn test110() {
    use std::ops::{Range, RangeInclusive};

    fn main() {
        assert_eq!((1..5), Range { start: 1, end: 5 });
        assert_eq!((1..=5), RangeInclusive::new(1, 5));

        println!("Success!");
    }

}

#[test]
fn test111() {
    fn main() {
        assert!(1u32 + 2 == 3);
        assert!(1i32 - 2 == -1);
        assert!(1u8.wrapping_sub(2) == 255);
        assert!(3 * 50 == 150);
        assert!(9.6 / 3.2 == 3.0);
        assert!(24 % 5 == 4);
        assert!(true && false == false);
        assert!(true || false == true);
        assert!(!true == false);
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    }

}

#[test]
fn test21() {
    use std::mem::size_of;
    fn main() {
        let c1 = 'a';
        assert_eq!(size_of::<char>(), 4);

        let c2 = '中';
        assert_eq!(size_of::<char>(), 4);

        println!("Success!");
    }

}

#[test]
fn test22() {
    fn main() {
        let c1 = "中";
        print_char(c1.chars().next().unwrap());
    }

    fn print_char(c: char) {
        println!("{}", c);
    }

}

#[test]
fn test23() {
    fn main() {
        let _f: bool = false;

        let t = false; // Change true to false
        if !t {
            println!("Success!");
        }
    }

}

#[test]
fn test24() {
    fn main() {
        let f = true;
        let t = true && true; // Change to true && true
        assert_eq!(t, f);

        println!("Success!");
    }

}

#[test]
fn test25() {
    fn main() {
        let _v: () = ();

        let v = (2, 3);
        assert_eq!(v, (2, 3));

        implicitly_ret_unit();

        println!("Success!");
    }

    fn implicitly_ret_unit() {
        println!("I will return a ()");
    }

}

#[test]
fn test26() {
    use std::mem::size_of_val;
    fn main() {
        let unit: () = ();
        assert!(size_of_val(&unit) == 0);

        println!("Success!");
    }
}

#[test]
fn test31() {
    fn main() {
        let v = {
            let mut x = 1;
            x += 2;
            x
        };

        assert_eq!(v, 3);

        println!("Success!");
    }

}

#[test]
fn test32() {
    fn main() {
        let v = {
            let x = 3;
            x
        };

        assert!(v == 3);

        println!("Success!");
    }

}

#[test]
fn test33() {
    fn main() {
        let s = sum(1, 2);
        assert_eq!(s, 3);

        println!("Success!");
    }

    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }

}

#[test]
fn test41() {
    fn main() {
        let (x, y) = (1, 2);
        let s = sum(x, y);

        assert_eq!(s, 3);

        println!("Success!");
    }

    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }

}

#[test]
fn test42() {
    fn main() {
        print();
    }
    fn print() -> () {
        println!("Success!");
    }

}

#[test]
fn test43() {
    fn main1() {
        never_return1();

        println!("Failed!");
    }

    fn never_return1() -> ! {
        loop {} // Infinite loop
    }

    /// или
    fn main2() {
        never_return2();

        println!("Failed!");
    }

    fn never_return2() -> ! {
        panic!("This function will never return!");
    }

}

#[test]
fn test44() {
    fn main() {
        println!("Success!");
    }

    fn get_option(tp: u8) -> Option<i32> {
        match tp {
            1 => {
                Some(42)
            }
            _ => {
                never_return_fn()
            }
        }
    }

    fn never_return_fn() -> ! {
        loop {}
    }
}

#[test]
fn test45() {
    fn main() {
        let b = true;
        let _v = match b {
            true => 1,
            false => {
                println!("Success!");
                panic!("we have no value for `false`, but we can panic");
            }
        };

        println!("Exercise Failed if printing out this line!");
    }

}