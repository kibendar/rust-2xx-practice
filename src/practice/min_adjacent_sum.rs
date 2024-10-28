use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_sum, min_index, min_index + 1)
}

fn print_min_adjacent(data: &[i32]) {
    println!("indexes: {}", (0..data.len()).map(|i| format!("{:>3}", i)).collect::<Vec<_>>().join(" "));
    println!("data:    {}", data.iter().map(|&x| format!("{:>3}", x)).collect::<Vec<_>>().join(", "));

    let (min_sum, index1, index2) = min_adjacent_sum(data);

    let mut indices_line = String::new();
    for i in 0..data.len() {
        if i == index1 {
            indices_line.push_str(" \\__");
        } else if i == index2 {
            indices_line.push_str("__/ ");
        } else {
            indices_line.push_str("     ");
        }
    }
    println!("indexes: {}", indices_line);
    println!("min adjacent sum={} at indexes:{},{}", min_sum, index1, index2);
}

#[test]
fn test() {
    let data = gen_random_vector(20);
    print_min_adjacent(&data);
}
