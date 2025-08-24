// LeetCode 81 - Search in Rotated Sorted Array II
// Same as #33 but duplicates are allowed; return whether target exists.
// Approach: like #33, but when nums[lo] == nums[mid] == nums[hi] we cannot tell
// which half is sorted, so shrink both ends by one.
// Time: O(log n) average, O(n) worst case, Space: O(1)

fn search(nums: &[i32], target: i32) -> bool {
    let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let (l, m, h) = (nums[lo as usize], nums[mid as usize], nums[hi as usize]);
        if m == target {
            return true;
        }
        if l == m && m == h {
            lo += 1;
            hi -= 1;
        } else if l <= m {
            if l <= target && target < m { hi = mid - 1 } else { lo = mid + 1 }
        } else if m < target && target <= h {
            lo = mid + 1
        } else {
            hi = mid - 1
        }
    }
    false
}

fn main() {
    assert!(search(&[2, 5, 6, 0, 0, 1, 2], 0));
    assert!(!search(&[2, 5, 6, 0, 0, 1, 2], 3));
    assert!(search(&[1, 0, 1, 1, 1], 0));
    println!("ok");
}
