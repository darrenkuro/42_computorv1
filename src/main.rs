mod math;
mod parser;

use std::env;
fn main() -> Result<(), String> {
    let input = if env::args().len() == 2 {
        env::args().nth(1).unwrap()
    } else {
        return Err("Wrong arguments!".to_string());
    };

    //let input = env::args().nth(1).ok_or("Missing arguments!")?;

    parser::parser(&input);
    //println!("{input}");
    Ok(())
}
