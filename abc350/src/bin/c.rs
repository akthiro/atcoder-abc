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
    }

    let mut result = vec![];

    let mut a = A.iter().map(|n| n - 1).collect::<Vec<_>>();

    let mut b = vec![0; N];

    for (i, n) in a.iter().enumerate() {
        b[*n] = i;
    }

    for i in 0..N - 1 {
        if a[i] == i {
            continue;
        }

        let j = b[i];

        a.swap(i, j);
        b.swap(a[i], a[j]);

        result.push((i + 1, j + 1));
    }

    if result.is_empty() {
        "0".to_string()
    } else {
        format!(
            "{}\n{}",
            result.len(),
            result
                .iter()
                .map(|(i, j)| format!("{} {}", i, j))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5
3 4 1 2 5
"#
            ),
            r#"2
1 3
2 4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4
1 2 3 4
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"3
3 1 2
"#
            ),
            r#"2
1 2
2 3"#
        );
    }
}
