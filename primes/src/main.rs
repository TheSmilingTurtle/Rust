use text_io::read;
use std::io;
use std::io::prelude::*;
use std::time::Instant;

fn main() {

    print!("Enter a number: ");

    let _ = io::stdout().flush();
    
    let number: usize = read!();

    let mut dirty_vec: Vec::<usize> = (2..=number).collect();

    let now = Instant::now();

    let lim  = (number as f64).sqrt().ceil() as usize;

    for i in 0..lim{
        let jump = dirty_vec[i];

        if jump != 0{
            for j in ((i+jump)..(number-1)).step_by(jump){
                dirty_vec[j] = 0;
            }
        }
    }

    let clean_vec: Vec<usize> = dirty_vec.into_iter().filter(|&x| x != 0).collect();

    let time = now.elapsed().as_millis();

    println!("");
    
    if number <= 100{
        println!("Here are your primes: \n{:?}", clean_vec);
    }

    println!("There are {} primes", clean_vec.len());

    println!("It took {}ms", time);
}