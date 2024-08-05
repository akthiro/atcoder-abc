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
        N: usize,
    }

    let mut result = vec![];

    for i in 0..N {
        if (i + 1) % 3 == 0 {
            result.push('x');
        } else {
            result.push('o');
        }
    }

    result.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"7
"#
            ),
            r#"ooxooxo"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"9
"#
            ),
            r#"ooxooxoox"#
        );
    }
}
