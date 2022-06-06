pub mod vectors;

#[cfg(test)]
mod tests {
    use super::vectors::Vector;

    #[test]
    fn it_works() {
        let a: Vector = Vector::new(
            3, 
            4, 
            -2
        );

        println!("{}", a.norm());
    }
}
