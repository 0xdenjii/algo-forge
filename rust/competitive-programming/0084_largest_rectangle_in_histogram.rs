// Problem 84: Largest Rectangle in Histogram
// Given bar heights of width 1, find the area of the largest rectangle in the histogram.
// Approach: Monotonic increasing stack; when a bar is popped its width spans from the new
// stack top + 1 to the current index - 1. A sentinel height 0 flushes the stack at the end.
// Complexity: O(n) time, O(n) space.
pub fn largest_rectangle_area(h: &[i32]) -> i64 {
    let mut st: Vec<usize> = Vec::new();
    let mut best = 0i64;
    for i in 0..=h.len() {
        let cur = if i == h.len() { 0 } else { h[i] };
        while let Some(&t) = st.last() {
            if h[t] < cur {
                break;
            }
            st.pop();
            let left = st.last().map_or(0, |&x| x + 1);
            best = best.max(h[t] as i64 * (i - left) as i64);
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
