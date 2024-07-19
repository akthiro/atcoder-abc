#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    collections::HashMap,
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
        ST: [(String, usize); N],
    }

    let mut map = HashMap::new();

    for (i, (s, t)) in ST.iter().enumerate() {
        map.entry(s.clone()).or_insert((i, *t));
    }

    let mut st = map.into_iter().collect::<Vec<_>>();
    st.sort_by(|a, b| b.1 .1.cmp(&a.1 .1).then(a.1 .0.cmp(&b.1 .0)));

    (st.first().unwrap().1 .0 + 1).to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
aaa 10
bbb 20
aaa 30
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
aaa 9
bbb 10
ccc 10
ddd 10
bbb 11
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10
bb 3
ba 1
aa 4
bb 1
ba 5
aa 9
aa 2
ab 6
bb 5
ab 3
"#
            ),
            r#"8"#
        );
    }
}
