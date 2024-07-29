#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        A: f64,
        B: f64,
        H: f64,
        M: f64,
    }

    let pi = (-1.0_f64).acos();

    let alpha = (H * 60.0 + M) / 720.0 * (pi * 2.0);
    let beta = M / 60.0 * (pi * 2.0);
    let theta = alpha - beta;

    let result = ((A * A) + (B * B) - (2.0 * A * B * theta.cos())).sqrt();

    format!("{:.20}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 4 9 0
"#
            ),
            r#"5.00000000000000000000"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 4 10 40
"#
            ),
            r#"4.56425719433005561143"#
        );
    }
}
