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
        N: usize,
        A: [usize; N],
    }

    let mut a = A;
    a.sort();

    let mut cnt = 0;
    let mut result = 0;

    let mut r = N;

    for i in 0..N {
        r = cmp::max(r, i + 1);

        while r - 1 > i && a[r - 1] + a[i] >= 100000000 {
            r -= 1;
        }

        cnt += N - r;
    }

    for n in a {
        result += n * (N - 1);
    }

    result -= cnt * 100000000;

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
3 50000001 50000002
"#
            ),
            r#"100000012"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
1 3 99999999 99999994 1000000
"#
            ),
            r#"303999988"#
        );
    }
}
