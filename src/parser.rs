pub struct PolyEq {
    degree: u8,
    zero: f32,
    one: f32,
    two: f32,
    discriminant: f32,
    x1: f32,
    x2: f32,
}

pub struct PolySolution {}

impl PolyEq {
    pub fn new() -> Self {
        Self {
            degree: 0,
            zero: 0f32,
            one: 0f32,
            two: 0f32,
            discriminant: 0f32,
            x1: 0f32,
            x2: 0f32,
        }
    }
}

pub fn parser(args: &str) -> Option<PolyEq> {
    let mut eq = PolyEq::new();
    let iter = args.split_whitespace();
    for str in iter {
        println!("iter: {str}");
        match str {
            s if s.starts_with('X') => {}
            _ => return None,
        }
    }
    // for c in args.chars() {
    // 	match c {
    // 		' ' => (),

    // 	}
    // }

    return Some(eq);
}
