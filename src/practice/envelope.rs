#[test]
fn main() {
    const WIDTH: usize = 30;  // Ширина конверта
    const HEIGHT: usize = 20; // Висота конверта

    let mut output = String::new();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || i == HEIGHT - 1 || j == 0 || j == WIDTH - 1 {
                output.push('*');
            } else if i == j * HEIGHT / WIDTH || i == (HEIGHT - 1) - j * HEIGHT / WIDTH {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}
