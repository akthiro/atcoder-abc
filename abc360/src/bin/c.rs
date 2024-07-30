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
        A: [usize; N],
        W: [usize; N],
    }

    let mut v = vec![vec![]; N];

    for (a, w) in A.iter().zip(W.iter()).collect::<Vec<_>>() {
        v[a - 1].push(*w);
    }

    let mut result = 0;

    for ws in v {
        if ws.len() <= 1 {
            continue;
        }

        let mut ws = ws;
        ws.sort();

        result += ws.iter().take(ws.len() - 1).sum::<usize>();
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
                r#"5
2 2 3 3 5
33 40 2 12 16
"#
            ),
            r#"35"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"12
3 6 7 4 12 4 8 11 11 1 8 11
3925 9785 9752 3587 4013 1117 3937 7045 6437 6208 3391 6309
"#
            ),
            r#"17254"#
        );
    }
}
