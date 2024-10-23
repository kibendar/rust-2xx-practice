use std::io;
use std::iter;

fn draw_tree(levels: u32) {
    let mut width = 1;
    for level in 0..levels {
        let height = level + 3;
        for i in 0..height {
            let spaces = (levels * 2 - 1) - (width / 2);
            let stars = width;
            // Друк пробілів і зірочок
            println!(
                "{}{}",
                iter::repeat(" ").take(spaces as usize).collect::<String>(),
                iter::repeat("*").take(stars as usize).collect::<String>()
            );
            width += 2;
        }
        // Після кожного рівня зменшуємо ширину на 4, щоб зробити наступний трикутник вужчим
        width -= 4;
    }
}

fn main() {
    // Введення користувачем кількості рівнів ялинки
    println!("Введіть кількість трикутників для ялинки:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка при читанні вводу");
    let levels: u32 = input.trim().parse().expect("Введіть ціле число");

    draw_tree(levels);
}
