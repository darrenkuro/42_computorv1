use crate::math::*; // maybe scope down

pub fn parser(args: &str) -> Result<Polynomial, String> {
    let mut poly = Polynomial::new();
    let args = args.split_whitespace();

    for arg in args {
        println!("iter: {arg}");
    }
    // let mut a = 0f32; // change name
    // let iter = args.split_whitespace();
    // // what if you need to check the format?
    // for str in iter {
    //     println!("iter: {str}");
    //     match str {
    //         '*' => (),
    //         '+' => (),
    //         '-' => (), // change to neg
    //         s if s.starts_with('X') => {}
    //         _ => (),
    //     }
    // }

    Ok(poly)
}
