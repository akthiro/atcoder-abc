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
        X: usize,
        Y: usize,
        Z: usize,
        A: [usize; N],
        B: [usize; N],
    }

    let mut result = vec![];

    let mut selected = vec![];

    let mut a = A.iter().enumerate().collect::<Vec<_>>();
    a.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(&b.0)));

    for (n, _) in a.iter().take(X) {
        result.push(*n);
        selected.push(*n)
    }

    let mut b = B
        .iter()
        .enumerate()
        .filter(|(i, _)| !selected.contains(i))
        .collect::<Vec<_>>();
    b.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(&b.0)));

    for (n, _) in b.iter().take(Y) {
        if selected.contains(n) {
            continue;
        }

        result.push(*n);
        selected.push(*n)
    }

    let mut c = A
        .iter()
        .zip(B.iter())
        .enumerate()
        .filter(|(i, _)| !selected.contains(i))
        .map(|(i, (a, b))| (i, (a + b)))
        .collect::<Vec<_>>();
    c.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    for (n, _) in c.iter().take(Z) {
        if selected.contains(n) {
            continue;
        }

        result.push(*n);
        selected.push(*n)
    }

    result.sort();

    result
        .iter()
        .map(|n| format!("{}", n + 1))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"6 1 0 2
80 60 80 60 70 70
40 20 50 90 90 80
"#
            ),
            r#"1
4
5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5 2 1 2
0 100 0 100 0
0 0 100 100 0
"#
            ),
            r#"1
2
3
4
5"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"15 4 3 2
30 65 20 95 100 45 70 85 20 35 95 50 40 15 85
0 25 45 35 65 70 80 90 40 55 20 20 45 75 100
"#
            ),
            r#"2
4
5
6
7
8
11
14
15"#
        );
    }
}
