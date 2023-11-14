pub struct ComplexNumber {
    real: f32,
    imaginary: f32,
}

impl ComplexNumber {
    pub fn new(real: f32, imaginary: f32) -> Self {
        Self { real, imaginary }
    }
}

impl std::fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (self.real, self.imaginary) {
            (r, i) if (r - 0.0).abs() < f32::EPSILON && (i - 0.0).abs() < f32::EPSILON => {
                write!(f, "0")
            }
            (r, i) if (i - 0.0).abs() < f32::EPSILON => write!(f, "{r}"), // i = 0
            (r, i) if (r - 0.0).abs() < f32::EPSILON => write!(f, "{i}i"), // r = 0
            (r, i) if i.is_sign_positive() => write!(f, "{} + {}i", r, i),
            (r, i) if i.is_sign_negative() => write!(f, "{} - {}i", r, i),
            (_, _) => unreachable!(),
        }
    }
}
