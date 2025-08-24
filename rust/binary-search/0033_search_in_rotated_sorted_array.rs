// LeetCode 33 - Search in Rotated Sorted Array
// A sorted array of distinct values was rotated at an unknown pivot. Return
// the index of target or -1, in O(log n).
// Approach: at each step one half is sorted; check whether target lies in it.
// Time: O(log n), Space: O(1)

fn search(nums: &[i32], target: i32) -> i32 {
    let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let (l, m, h) = (nums[lo as usize], nums[mid as usize], nums[hi as usize]);
        if m == target {
            return mid;
        }
        if l <= m {
            if l <= target && target < m { hi = mid - 1 } else { lo = mid + 1 }
        } else if m < target && target <= h {
            lo = mid + 1
        } else {
            hi = mid - 1
        }
    }
    -1
}

fn main() {
    assert_eq!(search(&[4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(search(&[4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(search(&[1], 0), -1);
    assert_eq!(search(&[3, 1], 1), 1);
    println!("ok");
}
