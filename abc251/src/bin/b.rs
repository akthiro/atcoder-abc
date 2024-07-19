#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    collections::HashSet,
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
        W: usize,
        A: [usize; N],
    }

    let mut set = HashSet::new();

    for n in A.iter() {
        if *n <= W {
            set.insert(*n);
        }
    }

    for i in 0..N {
        if A[i] > W {
            continue;
        }

        for j in i + 1..N {
            let sum = A[i] + A[j];

            if sum <= W {
                set.insert(sum);
            }
        }
    }

    for i in 0..N {
        if A[i] > W {
            continue;
        }

        for j in i + 1..N {
            if A[i] + A[j] > W {
                continue;
            }

            for k in j + 1..N {
                let sum = A[i] + A[j] + A[k];

                if sum <= W {
                    set.insert(sum);
                }
            }
        }
    }

    set.len().to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2 10
1 3
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2 1
2 3
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"4 12
3 3 3 3
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"7 251
202 20 5 1 4 2 100
"#
            ),
            r#"48"#
        );
    }
}
