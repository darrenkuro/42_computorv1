pub mod polynomial;
pub mod term;

pub fn sqrt(x: f32) -> f32 {
    if x < 0.0 {
        panic!("Cannot compute the square root of a negative number");
    }

    let mut guess = x / 2.0;

    // Divide by x to use relative tolerance to prevent infinite loop
    while (guess * guess - x).abs() / x > 1e-6 {
        guess = (guess + x / guess) / 2.0;
    }

    guess
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_test() {
        assert_eq!(sqrt(0f32), 0f32);
        assert_eq!(sqrt(4f32), 2f32);
        assert_eq!(sqrt(9f32), 3f32);
    }
}
