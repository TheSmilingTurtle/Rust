use std::env;

fn fibo(i: usize, j: usize, lim: usize) -> String {
    if i >= lim {
        return "".to_string()
    }
    format!(", {}{}",i, fibo(j, i+j, lim))
}

fn main() {
    let number: usize = env::args()
                            .collect::<Vec<String>>()[1]
                            .to_owned()
                            .parse()
                            .ok()
                            .expect("Could not convert to type u64.");

                        
    println!("[0{}]", fibo(1, 1, number));
}