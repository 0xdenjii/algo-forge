// LeetCode 162 - Find Peak Element
// Return the index of any element strictly greater than its neighbours, with
// nums[-1] = nums[n] = -infinity. Adjacent elements differ. O(log n) required.
// Approach: if nums[mid] < nums[mid+1] a peak exists to the right, else at/left of mid.
// Time: O(log n), Space: O(1)

fn find_peak_element(nums: &[i32]) -> usize {
    let (mut lo, mut hi) = (0, nums.len() - 1);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid] < nums[mid + 1] {
            lo = mid + 1
        } else {
            hi = mid
        }
    }
    lo
}

fn main() {
    assert_eq!(find_peak_element(&[1, 2, 3, 1]), 2);
    let p = find_peak_element(&[1, 2, 1, 3, 5, 6, 4]);
    assert!(p == 1 || p == 5);
    assert_eq!(find_peak_element(&[1]), 0);
    println!("ok");
}
