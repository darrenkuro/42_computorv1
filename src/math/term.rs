use std::fmt;

#[derive(Debug)]
pub struct Term {
    pub degree: u8,
    pub coefficient: f32,
}

impl Term {
    pub fn new(term: &str) -> Result<Self, String> {
        let components: Vec<&str> = term.split("* X^").collect();
        // If were to manage free form entries, do it here
        if components.len() != 2 {
            return Err(format!("Syntax Error: invalid term {term}!").to_string());
        }

        let c: Result<f32, _> = components[0].trim().parse();
        let d: Result<u8, _> = components[1].trim().parse();

        match (c, d) {
            (Ok(coefficient), Ok(degree)) => Ok(Self {
                degree,
                coefficient,
            }),
            _ => Err(format!("Syntax Error: invalid term {term}!")),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.coefficient >= 0.0 {
            write!(f, "{} * X^{}", self.coefficient, self.degree)
        } else {
            write!(f, "- {} * X^{}", -self.coefficient, self.degree)
        }
    }
}

#[cfg(test)]
mod term {
    #[test]
    fn print() {
        // let mut term = Term::new(10, 1.1);
        // let mut buf = Vec::new();
        // write!(buf, "{term}").unwrap();
        // assert_eq!(String::from_utf8(buf).unwrap(), "1.1 * X^10");

        // buf = Vec::new();
        // term = Term::new(10, 1.0);
        // write!(buf, "{term}").unwrap();
        // assert_eq!(String::from_utf8(buf).unwrap(), "1 * X^10");

        // buf = Vec::new();
        // term = Term::new(10, -1.0);
        // write!(buf, "{term}").unwrap();
        // assert_eq!(String::from_utf8(buf).unwrap(), "1 * X^10");
    }
}
