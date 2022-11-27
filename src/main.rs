mod vec_calc;
use std::{env, process::exit};

use crate::vec_calc::{vec_calc_get_promedio, vec_calc_get_mediana, vec_calc_get_modo};
fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut numbers:Vec<i32> = Vec::new();
    args.remove(0);
    args.iter().for_each(|s|{
        match s.trim().parse::<i32>(){
            Ok(n)=>numbers.push(n),
            Err(e)=>{
                eprintln!("invalidate params: {}", e);
                eprintln!("string: {}",s);
                exit(1);
            },
        };
    });
    println!("data: {:?}", numbers);
    println!("----resumen----");
    println!("promedio: {}", vec_calc_get_promedio(&numbers));
    println!("mediana: {}", vec_calc_get_mediana(&numbers));
    println!("modo: {}", vec_calc_get_modo(&numbers));
}
