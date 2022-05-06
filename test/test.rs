fn hello() -> ()
{
    println!("Hello World!");
}

fn read_lice(slice: [i32; 2])
{
    println!("slice.0: {}", slice[0]);
}

fn print_num(c: u32) -> Option<bool>
{
    if c != 5 {
        println!("{}", c);
        Some(true)
    }else{
        None
    }
}

#[derive(Debug)]
struct MyStruct {
    my_num1: i16,
    my_num2: i16
}

fn main()
{
    let mut c = 1_000_000;
    c = 2; //should throw an error, not anymore
    hello();
    if print_num(c) == None{
        println!("None");
    }
    println!("{:?}", MyStruct{my_num1: 32,my_num2: c as i16});

    let g: [i32; 2] = [4, 5];
    read_lice(g);
}