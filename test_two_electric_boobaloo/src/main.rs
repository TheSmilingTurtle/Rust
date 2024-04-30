use std::str::FromStr;

fn main() 
{
    println!("Enter something: ");

    let mut line = String::new();
    let _ = std::io::stdin()
                    .read_line(&mut line)
                    .ok()
                    .expect("Could not read the line.");

    let mut fac: f64 = 2f64;
    let num: f64 = u8::from_str( &line.trim()[..] ).ok().unwrap() as f64;
    let mut count: f64 = 3f64;

    let now = std::time::Instant::now();

    while count <= num {
        fac *= count;
        count += 1f64;
    }

    let time = now.elapsed().as_nanos();
    
    println!("The Factorial is: {:e}", fac);

    println!("It took {} ns", time);
}