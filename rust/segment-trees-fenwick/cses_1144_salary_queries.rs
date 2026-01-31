// CSES 1144 - Salary Queries
// Problem: n employees have salaries. Process "! k x" (employee k's salary
//   becomes x) and "? a b" (how many employees earn between a and b,
//   inclusive). Salaries go up to 1e9.
// Approach: Offline coordinate compression over every salary that ever
//   appears (initial values plus updates). A Fenwick tree counts employees per
//   compressed salary. A query maps [a, b] to rank bounds with binary searches.
// Complexity: O((n + q) log(n + q)) time and O(n + q) space.

enum Query {
    Set(usize, i64),
    Count(i64, i64),
}

struct Fenwick(Vec<i32>);

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick(vec![0; n + 1])
    }
    fn add(&mut self, mut i: usize, d: i32) {
        while i < self.0.len() {
            self.0[i] += d;
            i += i & i.wrapping_neg();
        }
    }
    fn prefix(&self, mut i: usize) -> i32 {
        let mut s = 0;
        while i > 0 {
            s += self.0[i];
            i &= i - 1;
        }
        s
    }
}

fn salary_queries(mut salaries: Vec<i64>, queries: &[Query]) -> Vec<i32> {
    let mut vals = salaries.clone();
    for q in queries {
        if let Query::Set(_, x) = q {
            vals.push(*x);
        }
    }
    vals.sort_unstable();
    vals.dedup();
    let rank = |x: i64| vals.binary_search(&x).unwrap() + 1;

    let mut bit = Fenwick::new(vals.len());
    for &s in &salaries {
        bit.add(rank(s), 1);
    }
    let mut out = Vec::new();
    for q in queries {
        match *q {
            Query::Set(k, x) => {
                bit.add(rank(salaries[k]), -1);
                salaries[k] = x;
                bit.add(rank(x), 1);
            }
            Query::Count(a, b) => {
                let hi = vals.partition_point(|&v| v <= b);
                let lo = vals.partition_point(|&v| v < a);
                out.push(if hi > lo { bit.prefix(hi) - bit.prefix(lo) } else { 0 });
            }
        }
    }
    out
}

fn main() {
    // Official sample (employee indices 1-based in the statement)
    let queries = [Query::Count(2, 3), Query::Set(2, 6), Query::Count(2, 3)];
    assert_eq!(salary_queries(vec![3, 7, 2, 2, 5], &queries), vec![3, 2]);
    let queries = [Query::Count(100, 200), Query::Set(0, 150), Query::Count(1, 1_000_000_000)];
    assert_eq!(salary_queries(vec![1, 2], &queries), vec![0, 2]);
    println!("CSES Salary Queries ok");
}
