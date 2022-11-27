mod vec_calc;

fn main() {
    let v = vec![1, 2, 3, 4];
    println!("{}", vec_calc::vec_calc_get_promedio(&v));
}
