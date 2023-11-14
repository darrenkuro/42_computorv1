use crate::math::polynomial::Polynomial; // maybe scope down
use crate::math::term::Term;

pub fn parse(args: &str) -> Result<Polynomial, String> {
    // Divide into left and right sides
    let sides: Vec<&str> = args.split('=').collect();
    if sides.len() != 2 {
        return Err("Syntax Error: no = symbol!".to_string());
    }

    // Reformat - sign for easy processing
    let lhs = sides[0].replace("- ", "+ -");
    let terms: Vec<&str> = lhs.split('+').collect();
    if terms.len() == 0 {
        return Err("Syntax Error: left side is empty!".to_string());
    }

    let mut lhs = Polynomial::new();
    for term in terms {
        match Term::new(term) {
            Ok(term) => lhs.push_term(term),
            Err(e) => return Err(e),
        }
    }

    let rhs = sides[1].replace("- ", "+ -");
    let terms: Vec<&str> = rhs.split('+').collect();
    if terms.len() == 0 {
        return Err("Syntax Error: right side is empty!".to_string());
    }
    let mut rhs = Polynomial::new();
    for term in terms {
        match Term::new(term) {
            Ok(term) => rhs.push_term(term),
            Err(e) => return Err(e),
        }
    }
    Ok(lhs - rhs)
}
