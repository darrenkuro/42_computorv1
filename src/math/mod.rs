pub mod polynomial;
pub mod term;

pub fn sqrt(x: f32) -> f32 {
    if x < 0.0 {
        panic!("Cannot compute the square root of a negative number");
    }

    let mut guess = x / 2.0;

    while (guess * guess - x).abs() > 1e-6 {
        guess = (guess + x / guess) / 2.0;
    }

    guess
}
