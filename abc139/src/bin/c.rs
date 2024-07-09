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
        H: [usize; N],
    }

    let mut checked = vec![false; N];

    let mut result = 0;

    for i in 0..N {
        if checked[i] {
            continue;
        }

        for j in i + 1..N {
            if H[j - 1] < H[j] {
                break;
            }

            checked[j] = true;
            result = cmp::max(result, j - i);
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
                r#"5
10 4 8 7 3
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"7
4 4 5 6 6 5 5
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"4
1 2 3 4
"#
            ),
            r#"0"#
        );
    }
}
