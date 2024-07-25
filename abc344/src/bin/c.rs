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
        A: [usize; N],
        M: usize,
        B: [usize; M],
        L: usize,
        C: [usize; L],
        Q: usize,
        X: [usize; Q],
    }

    let mut set = HashSet::new();

    for a in A.iter() {
        for b in B.iter() {
            for c in C.iter() {
                set.insert(a + b + c);
            }
        }
    }

    let mut result = vec![];

    for x in X {
        if set.contains(&x) {
            result.push("Yes");
        } else {
            result.push("No");
        }
    }

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
1 2 3
2
2 4
6
1 2 4 8 16 32
4
1 5 10 50
"#
            ),
            r#"No
Yes
Yes
No"#
        );
    }
}
