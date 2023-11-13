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
            (0f32, 0f32) => write!(f, "0"),
            (_, 0f32) => write!(f, "{}", self.real),
            (0f32, _) => write!(f, "{}i", self.imaginary),
            (r, i) if i.is_sign_positive() => write!(f, "{} + {}i", self.real, self.imaginary),
            (r, i) if i.is_sign_negative() => write!(f, "{} - {}i", self.real, self.imaginary),
        }
    }
}
