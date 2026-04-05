// Title: Fast I/O Scanner
// Problem: Read many whitespace-separated tokens from stdin quickly and write output with a
// buffered writer, as required by most Codeforces / AtCoder problems.
// Approach: Read all of stdin once, split on ASCII whitespace, parse lazily via generic FromStr.
// Complexity: O(input size) time and space.
use std::io::{self, BufWriter, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

pub struct Scanner<'a> {
    it: SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { it: s.split_ascii_whitespace() }
    }
    pub fn next<T: FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.it.next().expect("unexpected EOF").parse().expect("parse error")
    }
    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T>
    where
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.next()).collect()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    if input.trim().is_empty() {
        input = "3\n1 2 3".into(); // demo input
    }
    let mut sc = Scanner::new(&input);
    let out = io::stdout();
    let mut w = BufWriter::new(out.lock());
    let n: usize = sc.next();
    let a: Vec<i64> = sc.vec(n);
    writeln!(w, "{}", a.iter().sum::<i64>()).unwrap();
}
