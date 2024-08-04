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
        H: usize,
    }

    let mut cur = 1;
    let mut result = 1;

    loop {
        if cur > H {
            break;
        }

        cur += 2_usize.pow(result);
        result += 1;
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
                r#"54
"#
            ),
            r#"6"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"7
"#
            ),
            r#"4"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"262144
"#
            ),
            r#"19"#
        );
    }
}
