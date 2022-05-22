
fn main() {
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).ok().expect("something went wrong");
    println!("Converted from binary: {}", u32::from_str_radix(&line.trim(), 2).ok().expect("something went wrong"));
}
