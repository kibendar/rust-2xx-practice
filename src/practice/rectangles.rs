#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(rectangles: &Vec<Rectangle>) -> i32 {
    use std::collections::HashSet;
    let mut covered_points = HashSet::new();

    for rect in rectangles {
        let x_min = rect.a.x.min(rect.b.x);
        let x_max = rect.a.x.max(rect.b.x);
        let y_min = rect.a.y.min(rect.b.y);
        let y_max = rect.a.y.max(rect.b.y);

        // Add each point in the rectangle to the set
        for x in x_min..x_max {
            for y in y_min..y_max {
                covered_points.insert((x, y));
            }
        }
    }

    // The total occupied area is the number of unique points covered
    covered_points.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
    println!("Test passed with occupied area: {}", occupied);
}

#[test]
fn main() {
    area_occupied_test();
}
