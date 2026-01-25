// LeetCode 1649 - Create Sorted Array through Instructions
// Problem: Insert instructions[i] one by one into a sorted container. Each
//   insertion costs min(#elements strictly less, #elements strictly greater).
//   Return the total cost modulo 1e9 + 7.
// Approach: A Fenwick tree indexed by value (values <= 1e5) counts the
//   inserted elements. less = prefix(x - 1); greater = inserted - prefix(x).
// Complexity: O(n log M) time, O(M) space, where M is the max value.

const MOD: i64 = 1_000_000_007;

struct Fenwick(Vec<i32>);

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick(vec![0; n + 1])
    }
    fn add(&mut self, mut i: usize) {
        while i < self.0.len() {
            self.0[i] += 1;
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

fn create_sorted_array(instructions: Vec<i32>) -> i32 {
    let max = instructions.iter().copied().max().unwrap_or(0) as usize;
    let mut bit = Fenwick::new(max);
    let mut cost: i64 = 0;
    for (inserted, &x) in instructions.iter().enumerate() {
        let x = x as usize;
        let less = bit.prefix(x - 1);
        let greater = inserted as i32 - bit.prefix(x);
        cost = (cost + less.min(greater) as i64) % MOD;
        bit.add(x);
    }
    cost as i32
}

fn main() {
    assert_eq!(create_sorted_array(vec![1, 5, 6, 2]), 1);
    assert_eq!(create_sorted_array(vec![1, 2, 3, 6, 5, 4]), 3);
    assert_eq!(create_sorted_array(vec![1, 3, 3, 3, 2, 4, 2, 1, 2]), 4);
    println!("1649 ok");
}
