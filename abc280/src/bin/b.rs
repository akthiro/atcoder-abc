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
        S: [isize; N],
    }

    let mut result = vec![S[0]];

    for i in 1..N {
        result.push(S[i] - S[i - 1]);
    }

    result
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
3 4 8
"#
            ),
            r#"3 1 4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10
314159265 358979323 846264338 -327950288 419716939 -937510582 97494459 230781640 628620899 -862803482
"#
            ),
            r#"314159265 44820058 487285015 -1174214626 747667227 -1357227521 1035005041 133287181 397839259 -1491424381"#
        );
    }
}
