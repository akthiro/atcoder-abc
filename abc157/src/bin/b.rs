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
        A: [[usize; 3]; 3],
        N: usize,
        b: [usize; N],
    }

    let mut a = A;

    for n in b {
        for rows in a.iter_mut() {
            for cell in rows.iter_mut() {
                if *cell == n {
                    *cell = 0;
                }
            }
        }
    }

    for i in 0..3 {
        if (a[i][0] == 0 && a[i][1] == 0 && a[i][2] == 0)
            || (a[0][i] == 0 && a[1][i] == 0 && a[2][i] == 0)
        {
            return "Yes".to_string();
        }
    }

    if (a[0][0] == 0 && a[1][1] == 0 && a[2][2] == 0)
        || (a[0][2] == 0 && a[1][1] == 0 && a[2][0] == 0)
    {
        return "Yes".to_string();
    }

    "No".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"84 97 66
79 89 11
61 59 7
7
89
7
87
79
24
84
30
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"41 7 46
26 89 2
78 92 8
5
6
45
16
57
17
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"60 88 34
92 41 43
65 73 48
10
60
43
88
11
48
73
65
41
92
34
"#
            ),
            r#"Yes"#
        );
    }
}
