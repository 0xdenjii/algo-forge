// LeetCode 287 - Find the Duplicate Number
// nums has n+1 integers in [1, n]; exactly one value repeats (possibly many
// times). Find it without modifying the array and using O(1) extra space.
// Approach: binary search on the value v: if count(x <= v) > v (pigeonhole),
// the duplicate is <= v.
// Time: O(n log n), Space: O(1)

fn find_duplicate(nums: &[i32]) -> i32 {
    let (mut lo, mut hi) = (1, nums.len() as i32 - 1);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let cnt = nums.iter().filter(|&&x| x <= mid).count() as i32;
        if cnt > mid { hi = mid } else { lo = mid + 1 }
    }
    lo
}

fn main() {
    assert_eq!(find_duplicate(&[1, 3, 4, 2, 2]), 2);
    assert_eq!(find_duplicate(&[3, 1, 3, 4, 2]), 3);
    assert_eq!(find_duplicate(&[3, 3, 3, 3, 3]), 3);
    println!("ok");
}
