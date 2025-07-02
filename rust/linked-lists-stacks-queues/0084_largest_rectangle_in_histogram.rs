// LeetCode 84 - Largest Rectangle in Histogram
// Given bar heights (width 1 each), find the area of the largest rectangle.
// Approach: monotonic increasing stack of indices. When a lower bar arrives, pop;
// the popped bar extends from the new stack top + 1 to i - 1. A sentinel 0 flushes.
// Time O(n), Space O(n).
pub fn largest_rectangle_area(h: &[i32]) -> i64 {
    let mut st: Vec<usize> = Vec::new();
    let mut best = 0i64;
    for i in 0..=h.len() {
        let cur = if i == h.len() { 0 } else { h[i] };
        while let Some(&top) = st.last() {
            if h[top] <= cur { break; }
            st.pop();
            let left = st.last().map_or(0, |&l| l + 1);
            best = best.max(h[top] as i64 * (i - left) as i64);
        }
        st.push(i);
    }
    best
}

fn main() {
    assert_eq!(largest_rectangle_area(&[2, 1, 5, 6, 2, 3]), 10);
    assert_eq!(largest_rectangle_area(&[2, 4]), 4);
    assert_eq!(largest_rectangle_area(&[]), 0);
    println!("ok");
}
