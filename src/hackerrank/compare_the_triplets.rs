#[test]
fn main(){
    let array_a = [10, 15, 20];
    let array_b = [10, 8, 8];

    let mut array_a_res = 0;

    let mut array_b_res = 0;

    for i in 0..array_a.len() {
            if array_a[i] == array_b[i] {
                continue;
            }else if array_a[i] > array_b[i] {
                array_a_res += 1;
            }else {
                array_b_res += 1;
            }
    }

    println!("Result for array A: {}, and result for array B: {} ",array_a_res, array_b_res);
}