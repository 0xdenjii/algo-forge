// LeetCode 1352 - Product of the Last K Numbers
// add(num) appends a number (0..=100) to a stream; get_product(k) returns the product of the
// last k numbers (guaranteed to fit in i32).
// Approach: prefix products that reset on zero; if k reaches back past the latest zero the
// answer is 0, else prefix[n] / prefix[n - k].
// Time: O(1) per op. Space: O(n).
pub struct ProductOfNumbers {
    prefix: Vec<i64>, // prefix[0] = 1 sentinel; reset to [1] when a zero arrives
}

impl ProductOfNumbers {
    pub fn new() -> Self {
        Self { prefix: vec![1] }
    }

    pub fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix = vec![1];
        } else {
            let last = *self.prefix.last().unwrap();
            self.prefix.push(last * num as i64);
        }
    }

    pub fn get_product(&self, k: i32) -> i32 {
        let n = self.prefix.len();
        let k = k as usize;
        if k >= n {
            0
        } else {
            (self.prefix[n - 1] / self.prefix[n - 1 - k]) as i32
        }
    }
}

fn main() {
    let mut p = ProductOfNumbers::new();
    for x in [3, 0, 2, 5, 4] {
        p.add(x);
    }
    assert_eq!(p.get_product(2), 20);
    assert_eq!(p.get_product(3), 40);
    assert_eq!(p.get_product(4), 0);
    p.add(8);
    assert_eq!(p.get_product(2), 32);
    println!("product of last k ok");
}
