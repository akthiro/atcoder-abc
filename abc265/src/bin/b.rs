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
        M: usize,
        T: isize,
        A: [isize; N - 1],
        XY: [(usize, isize); M],
    }

    let mut ys = vec![0_isize; N - 1];

    for (x, y) in XY {
        ys[x - 2] = y;
    }

    let mut t = T;

    for (i, n) in A.iter().enumerate() {
        if t - n <= 0 {
            return "No".to_string();
        }

        t -= n;
        t += ys[i];
    }

    "Yes".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4 1 10
5 7 5
2 10
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4 1 10
10 7 5
2 10
"#
            ),
            r#"No"#
        );
    }
}
