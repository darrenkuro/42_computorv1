mod math;
mod parser;

use std::env;
fn main() -> Result<(), String> {
    let input = if env::args().len() == 2 {
        env::args().nth(1).unwrap()
    } else {
        return Err("Wrong arguments!".to_string());
    };

    match parser::parse(&input) {
        Ok(mut poly) => {
            println!("Reduced form: {}", poly.print_form());
            println!("Polynomial degree: {}", poly.get_degree());
            poly.try_solve();
            Ok(())
        }
        Err(e) => Err(e),
    }
}
