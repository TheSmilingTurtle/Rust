fn main() {
    let number: usize = {
        std::io::Write::write_all(&mut std::io::stdout(), b"Enter a number: ")
            .expect("Could not prompt user"); //prompt user
        std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout"); //flush stdout, to disply prompt

        let mut buffer = String::new(); //get a string ready
        let _ = std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line"); //read into string

        buffer
            .trim()
            .parse::<usize>()
            .expect("Could not parse input") //parse from string and store in number
    };

    if number == 0 {
        return;
    }

    let now = std::time::Instant::now(); //start the clock

    let clean_vec: Vec<usize> = {
        let mut dirty_vec: Vec<_> = (2..=number).collect(); //initialize vector

        let lim: usize = (number as f64).sqrt().floor() as usize - 1; // floor because a divisor needs to be smaller than the sqrt() of a number

        for pos in 0..lim {
            let jump = dirty_vec[pos];

            if jump != 0 {
                // jump = 0 means we have already visited this position
                dirty_vec
                    .iter_mut()
                    .skip(pos + jump) //skip to the next multiple of pos
                    .step_by(jump) //skip along the vector
                    .for_each(|elem| *elem = 0); //write everything you land to 0 */
            }
        }

        dirty_vec.into_iter().filter(|x| *x != 0).collect() //clean the vector ans tore in clean_vec
    };

    let time = now.elapsed().as_millis(); //read elapsed time

    /* Results printing */
    std::io::Write::write_all(&mut std::io::stdout(), b"\n").expect("Could create new line");
    if number <= 100 {
        std::io::Write::write_all(
            &mut std::io::stdout(),
            ("Here are your primes: \n".to_owned() + &format!("{:?}", clean_vec) + "\n").as_bytes(),
        )
        .expect("Could not post to stdout");
    }
    std::io::Write::write_all(
        &mut std::io::stdout(),
        ("There are ".to_owned() + &clean_vec.len().to_string() + " primes\n").as_bytes(),
    )
    .expect("Could not output count of primes");
    std::io::Write::write_all(
        &mut std::io::stdout(),
        ("It took ".to_owned() + &time.to_string() + "ms\n").as_bytes(),
    )
    .expect("Could not output duration");
}
