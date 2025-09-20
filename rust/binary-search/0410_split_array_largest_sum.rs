// LeetCode 410 - Split Array Largest Sum
// Split a non-negative array into k non-empty contiguous subarrays so that the
// largest subarray sum is minimised. Return that minimised largest sum.
// Approach: binary search the answer; greedily count pieces needed for a cap.
// Time: O(n log sum), Space: O(1)

fn pieces(nums: &[i32], cap: i64) -> i32 {
    let (mut count, mut cur) = (1, 0i64);
    for &x in nums {
        if cur + x as i64 > cap {
            count += 1;
            cur = 0;
        }
        cur += x as i64;
    }
    count
}

fn split_array(nums: &[i32], k: i32) -> i32 {
    let mut lo = *nums.iter().max().unwrap() as i64;
    let mut hi: i64 = nums.iter().map(|&x| x as i64).sum();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if pieces(nums, mid) <= k { hi = mid } else { lo = mid + 1 }
    }
    lo as i32
}

fn main() {
    assert_eq!(split_array(&[7, 2, 5, 10, 8], 2), 18);
    assert_eq!(split_array(&[1, 2, 3, 4, 5], 2), 9);
    assert_eq!(split_array(&[1, 4, 4], 3), 4);
    println!("ok");
}
