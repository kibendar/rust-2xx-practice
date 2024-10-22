#[test]
fn main() {

        const SIZE: usize = 5; // Висота верхньої частини ромба

        // Збираємо всі рядки ромба в один великий рядок
        let mut output = String::new();

        for i in 0..SIZE {
            output.push_str(&" ".repeat(SIZE - i - 1));
            output.push_str(&"*".repeat(2 * i + 1));
            output.push('\n');
        }

        for i in (0..SIZE - 1).rev() {
            output.push_str(&" ".repeat(SIZE - i - 1));
            output.push_str(&"*".repeat(2 * i + 1));
            output.push('\n');
        }

        // Використовуємо print! для виведення ромба
        print!("{}", output);

        // Використовуємо println! для переходу на новий рядок після виводу ромба
        println!();


    // const ROWS: i8 = 6;
    // let mut count = ROWS - 1;
    //
    // for i  in 1..= ROWS {
    //     for j in 1..= count {
    //         print!(" ");
    //     }
    //     count -= 1;
    //     for j in 1..= 2 * i - 1{
    //         print!("*");
    //     }
    //     println!(" ");
    // }
    //
    // count = 1;
    //
    // for i  in 1..= ROWS {
    //     for j in 1..=count {
    //         print!(" ");
    //     }
    //     count += 1;
    //
    //     for j in 1..= 2 * (ROWS - i) - 1 {
    //         print!("*")
    //     }
    //     println!(" ");
    // }
}