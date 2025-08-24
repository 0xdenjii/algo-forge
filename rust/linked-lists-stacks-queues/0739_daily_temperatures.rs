// LeetCode 739 - Daily Temperatures
// For each day, how many days until a warmer temperature? 0 if none.
// Approach: monotonic decreasing stack of indices; a warmer day resolves all
// cooler days on top of the stack.
// Time O(n), Space O(n).
pub fn daily_temperatures(t: &[i32]) -> Vec<usize> {
    let mut ans = vec![0; t.len()];
    let mut st: Vec<usize> = Vec::new();
    for (i, &x) in t.iter().enumerate() {
        while let Some(&j) = st.last() {
            if t[j] >= x { break; }
            ans[j] = i - j;
            st.pop();
        }
        st.push(i);
    }
    ans
}

fn main() {
    assert_eq!(daily_temperatures(&[73, 74, 75, 71, 69, 72, 76, 73]), vec![1, 1, 4, 2, 1, 1, 0, 0]);
    println!("ok");
}
