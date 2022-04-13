fn hello()
{
    println!("Hello World!");
}

fn print_num(c: u32) -> bool
{
    println!("{}", c);

    true
}

#[derive(Debug)]
struct MyStruct {
    my_num1: i16,
    my_num2: i16
}

fn main()
{
    let c;
    c = 1_000_000u32;
    hello();
    println!("{}", print_num(c));
    println!("{:?}", MyStruct{my_num1: 32,my_num2: 14});
}