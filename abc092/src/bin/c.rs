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
        A: [isize; N],
    }

    let mut result = vec![];

    let mut total = 0;
    let mut n = 0;

    for m in A.iter() {
        total += (n - m).abs();
        n = *m;
    }

    total += n.abs();

    for i in 0..N {
        let a = if i == 0 { 0 } else { A[i - 1] };
        let b = A[i];
        let c = if i == N - 1 { 0 } else { A[i + 1] };

        result.push(total - (a - b).abs() - (b - c).abs() + (a - c).abs());
    }

    result
        .iter()
        .map(|n| n.to_string())
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
                r#"3
3 5 -1
"#
            ),
            r#"12
8
10"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
1 1 1 2 0
"#
            ),
            r#"4
4
4
2
4"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"6
-679 -2409 -3258 3095 -3291 -4462
"#
            ),
            r#"21630
21630
19932
8924
21630
19288"#
        );
    }
}
