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
        A: usize,
        B: usize,
        D: [usize; N],
    }

    let mut days = D.iter().map(|d| d % (A + B)).collect::<Vec<_>>();
    days.sort();

    for i in 0..N {
        days.push(days[i] + A + B);
    }

    let mut min = usize::MAX;

    for i in 0..N {
        min = cmp::min(min, days[i + N - 1] - days[i] + 1);
    }

    if min <= A {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 2 5
1 2 9
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2 5 10
10 15
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"4 347 347
347 700 705 710
"#
            ),
            r#"Yes"#
        );
    }
}
