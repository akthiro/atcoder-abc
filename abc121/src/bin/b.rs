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
        C: isize,
        B: [isize; M],
        A: [[isize; M]; N],
    }

    let mut result = 0;

    for n in A {
        let mut total = 0;

        for (i, m) in n.iter().enumerate() {
            total += m * B[i];
        }

        if total + C > 0 {
            result += 1;
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
                r#"2 3 -10
1 2 3
3 2 1
1 2 2
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5 2 -4
-2 5
100 41
100 40
-3 0
-6 -2
18 -13
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"3 3 0
100 -100 0
0 100 100
100 100 100
-100 100 100
"#
            ),
            r#"0"#
        );
    }
}
