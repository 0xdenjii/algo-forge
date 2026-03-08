// LeetCode 973 - K Closest Points to Origin
// Return the k points closest (Euclidean distance) to the origin, in any order.
// Approach: quickselect via select_nth_unstable_by_key on squared distance.
// Time: O(n) average | Space: O(1) extra
pub fn k_closest(mut pts: Vec<(i64, i64)>, k: usize) -> Vec<(i64, i64)> {
    if k < pts.len() {
        pts.select_nth_unstable_by_key(k, |&(x, y)| x * x + y * y);
        pts.truncate(k);
    }
    pts
}

fn main() {
    let r = k_closest(vec![(3, 3), (5, -1), (-2, 4)], 2);
    assert!(r.contains(&(3, 3)) && r.contains(&(-2, 4)));
    println!("{:?}", r);
}
