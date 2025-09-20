// LeetCode 719 - Find K-th Smallest Pair Distance
// The distance of pair (i, j), i < j, is |nums[i] - nums[j]|. Return the kth
// smallest distance among all pairs.
// Approach: sort, then binary search the distance d; count pairs with
// distance <= d using a sliding two-pointer window.
// Time: O(n log n + n log W), Space: O(1) besides sort

fn smallest_distance_pair(mut nums: Vec<i32>, k: usize) -> i32 {
    nums.sort_unstable();
    let count = |d: i32| {
        let (mut left, mut cnt) = (0usize, 0usize);
        for right in 0..nums.len() {
            while nums[right] - nums[left] > d {
                left += 1;
            }
            cnt += right - left;
        }
        cnt
    };
    let (mut lo, mut hi) = (0, nums[nums.len() - 1] - nums[0]);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if count(mid) >= k { hi = mid } else { lo = mid + 1 }
    }
    lo
}

fn main() {
    assert_eq!(smallest_distance_pair(vec![1, 3, 1], 1), 0);
    assert_eq!(smallest_distance_pair(vec![1, 1, 1], 2), 0);
    assert_eq!(smallest_distance_pair(vec![1, 6, 1], 3), 5);
    println!("ok");
}
