use std::fmt;
mod complex;
pub struct Term {
    degree: u8,
    coefficient: f32,
}

struct Solution {}

pub struct Polynomial {
    pub terms: Vec<Term>,
}

impl Term {
    pub fn new(degree: u8, coefficient: f32) -> Self {
        Self {
            degree,
            coefficient,
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.coefficient > 0.0 {
            write!(f, "{} * X^{}", self.coefficient, self.degree)
        } else if self.coefficient < 0.0 {
            write!(f, "- {} * X^{}", -self.coefficient, self.degree)
        } else {
            write!(f, "")
        }
    }
}

impl Polynomial {
    pub fn new() -> Self {
        Self { terms: Vec::new() }
    }

    pub fn push_term(&mut self, term: Term) {
        self.terms.push(term);
    }

    pub fn get_degree(&self) -> u8 {
        let mut degree = 0;
        for term in &self.terms {
            if term.coefficient != 0.0 && term.degree > degree {
                degree = term.degree;
            };
        }
        degree
    }

    pub fn print_form(&mut self) {
        self.terms.sort_by_key(|term| term.degree);
        for term in &mut self.terms {
            if term.degree == 0 || term.coefficient < 0f32 {
                print!("{term}");
            } else {
                print!("+ {term}");
            }
        }
        print!(" = 0");
    }

    fn solve_second_degree(&self) -> Result<Vec<f32>, String> {
        let get_discriminant = |poly: &Polynomial| -> Option<f32> {
            let (mut a, mut b, mut c) = (0f32, 0f32, 0f32);
            for term in &poly.terms {
                match term.degree {
                    2 => a = term.coefficient,
                    1 => b = term.coefficient,
                    0 => c = term.coefficient,
                    _ => (),
                }
            }
            Some(b * b - 4f32 * a * c)
        };

        match get_discriminant(self) {
            _ => {}
        }
    }

    // fn solve_first_degree(&self) -> Result<Vec<f32>, String> {}

    pub fn solve(&self) -> Result<Vec<f32>, String> {
        match self.get_degree() {
            2 => Ok(Vec::new()),
            1 => Ok(Vec::new()),
            0 => Err("Each real number is a solution.".to_string()),
            _ => Err("The polynomial degree is strigly greater than 2, I can't solve.".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn push_term() {
        let mut poly = Polynomial::new();
        poly.push_term(Term::new(10, 0.0));
        assert_eq!(poly.get_degree(), 0);

        poly = Polynomial::new();
        poly.push_term(Term::new(10, 1.0));
        assert_eq!(poly.get_degree(), 10);
    }

    #[test]
    fn print_term() {
        let mut term = Term::new(10, 1.1);
        let mut buf = Vec::new();
        write!(buf, "{term}").unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "1.1 * X^10");

        buf = Vec::new();
        term = Term::new(10, 1.0);
        write!(buf, "{term}").unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "1 * X^10");

        buf = Vec::new();
        term = Term::new(10, -1.0);
        write!(buf, "{term}").unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "1 * X^10");
    }
}
