// LeetCode 901 - Online Stock Span
// For each new daily price, return how many consecutive days (including today)
// the price was <= today's price.
// Approach: monotonic decreasing stack of (price, span); merge spans of popped entries.
// Time amortized O(1) per call, Space O(n).
#[derive(Default)]
pub struct StockSpanner { st: Vec<(i32, u32)> }

impl StockSpanner {
    pub fn new() -> Self { Self::default() }
    pub fn next(&mut self, price: i32) -> u32 {
        let mut span = 1;
        while let Some(&(p, s)) = self.st.last() {
            if p > price { break; }
            span += s;
            self.st.pop();
        }
        self.st.push((price, span));
        span
    }
}

fn main() {
    let mut s = StockSpanner::new();
    let got: Vec<u32> = [100, 80, 60, 70, 60, 75, 85].iter().map(|&p| s.next(p)).collect();
    assert_eq!(got, vec![1, 1, 1, 2, 1, 4, 6]);
    println!("ok");
}
