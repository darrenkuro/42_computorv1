mod parser;

use std::env;
fn main() -> Result<(), String> {
    let input = env::args().nth(1).ok_or("Missing arguments!")?;
    parser::parser(&input);
    println!("{input}");
    Ok(())
}
