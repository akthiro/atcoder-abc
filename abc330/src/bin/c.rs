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
        D: isize,
    }

    let mut result = D;

    for x in 0..=2000000 {
        if x * x >= D {
            result = result.min(x * x - D);
        } else {
            let y = ((D - x * x) as f64).sqrt() as isize;
            result = result.min((x * x + y * y - D).abs());
            result = result.min((x * x + (y + 1) * (y + 1) - D).abs());
        }
    }

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"21
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(solve(r#"998244353"#), r#"0"#);
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"264428617
"#
            ),
            r#"32"#
        );
    }
}
