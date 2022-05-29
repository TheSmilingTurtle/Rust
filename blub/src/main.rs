use std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut rng = rand::thread_rng();

    if rng.gen::<f32>() < 0.01{
        opener::open("https://www.youtube.com/watch?v=xvFZjo5PgG0").expect("Could not open the new content");
        drop(rng);
        drop(args);
        std::process::exit(0);
    }

    for _ in 1..args.len(){
        let rand = rng.gen::<f32>();
        if rand < 0.5{
            print!("blub ");
        }
        else{
            print!("bitch ");
        }
    }

    if args.len()==1
    {
        print!("blub");
    }
}