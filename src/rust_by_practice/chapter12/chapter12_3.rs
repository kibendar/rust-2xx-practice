/// https://practice.course.rs/type-conversions/others.html

use std::fmt;
use std::str::FromStr;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

// To use `from_str` method, you needs to introduce this trait into the current scope.

#[test]
fn test157() {
    let origin = Point { x: 0, y: 0 };
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success!")
}
fn test158() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!")
}

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point1 {
    x: i32,
    y: i32
}

impl FromStr for Point1 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
            .split(',')
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point1 { x: x_fromstr, y: y_fromstr })
    }
}

#[test]
fn test159() {
    let p = "(3,4)".parse::<Point1>();
    assert_eq!(p.unwrap(), Point1{ x: 3, y: 4} )
}