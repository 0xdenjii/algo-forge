// LeetCode 1395 - Count Number of Teams
// Problem: Soldiers stand in a line with distinct ratings. Count the triples
//   i < j < k whose ratings are strictly increasing or strictly decreasing.
// Approach: Treat each soldier j as the middle. Two Fenwick trees over
//   compressed ratings count the smaller/larger ratings to the left and to
//   the right. j contributes lessL*greaterR + greaterL*lessR.
// Complexity: O(n log n) time, O(n) space.

struct Fenwick(Vec<i64>);

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick(vec![0; n + 1])
    }
    fn add(&mut self, mut i: usize, d: i64) {
        while i < self.0.len() {
            self.0[i] += d;
            i += i & i.wrapping_neg();
        }
    }
    fn prefix(&self, mut i: usize) -> i64 {
        let mut s = 0;
        while i > 0 {
            s += self.0[i];
            i &= i - 1;
        }
        s
    }
}

fn num_teams(rating: Vec<i32>) -> i64 {
    let mut sorted = rating.clone();
    sorted.sort_unstable();
    let ranks: Vec<usize> = rating.iter().map(|x| sorted.binary_search(x).unwrap() + 1).collect();
    let n = ranks.len();
    let mut left = Fenwick::new(n);
    let mut right = Fenwick::new(n);
    for &r in &ranks {
        right.add(r, 1);
    }
    let mut total = 0;
    for (j, &r) in ranks.iter().enumerate() {
        right.add(r, -1);
        let less_l = left.prefix(r - 1);
        let greater_l = j as i64 - less_l;
        let less_r = right.prefix(r - 1);
        let greater_r = (n - 1 - j) as i64 - less_r;
        total += less_l * greater_r + greater_l * less_r;
        left.add(r, 1);
    }
    total
}

fn main() {
    assert_eq!(num_teams(vec![2, 5, 3, 4, 1]), 3);
    assert_eq!(num_teams(vec![2, 1, 3]), 0);
    assert_eq!(num_teams(vec![1, 2, 3, 4]), 4);
    println!("1395 ok");
}
