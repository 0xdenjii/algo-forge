// LeetCode 42 - Trapping Rain Water
// Given an elevation map, compute how much water is trapped after raining.
// Approach (stack): keep a decreasing stack of indices; when a taller bar arrives,
// pop the "floor" and fill the bounded layer between the new top and the current bar.
// Time O(n), Space O(n).
pub fn trap(h: &[i32]) -> i64 {
    let mut st: Vec<usize> = Vec::new();
    let mut water = 0i64;
    for (i, &x) in h.iter().enumerate() {
        while let Some(&floor) = st.last() {
            if h[floor] >= x { break; }
            st.pop();
            let Some(&left) = st.last() else { break };
            let height = h[left].min(x) - h[floor];
            water += height as i64 * (i - left - 1) as i64;
        }
        st.push(i);
    }
    water
}

fn main() {
    assert_eq!(trap(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(trap(&[4, 2, 0, 3, 2, 5]), 9);
    println!("ok");
}
