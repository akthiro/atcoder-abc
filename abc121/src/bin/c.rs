#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp,
    io::{self, Read},
};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        N: isize,
        M: isize,
        AB: [(isize, isize); N],
    }

    let mut ab = AB;
    ab.sort_by(|a, b| a.0.cmp(&b.0));

    let mut result = 0;

    let mut m = M;

    for (price, n) in ab {
        if m <= 0 {
            break;
        }

        let buy = cmp::min(m, n);
        m -= buy;
        result += buy * price;
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
                r#"2 5
4 9
2 4
"#
            ),
            r#"12"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4 30
6 18
2 5
3 10
7 9
"#
            ),
            r#"130"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1 100000
1000000000 100000
"#
            ),
            r#"100000000000000"#
        );
    }
}
