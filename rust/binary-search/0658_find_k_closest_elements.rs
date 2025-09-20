// LeetCode 658 - Find K Closest Elements
// Given a sorted array, k and x, return the k elements closest to x in sorted
// order; ties are broken in favour of the smaller element.
// Approach: binary search the left edge L of the window [L, L+k) in [0, n-k]:
// move right iff x - arr[mid] > arr[mid+k] - x.
// Time: O(log(n-k) + k), Space: O(1) besides output

fn find_closest_elements(arr: &[i32], k: usize, x: i32) -> Vec<i32> {
    let (mut lo, mut hi) = (0, arr.len() - k);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if x - arr[mid] > arr[mid + k] - x { lo = mid + 1 } else { hi = mid }
    }
    arr[lo..lo + k].to_vec()
}

fn main() {
    assert_eq!(find_closest_elements(&[1, 2, 3, 4, 5], 4, 3), vec![1, 2, 3, 4]);
    assert_eq!(find_closest_elements(&[1, 2, 3, 4, 5], 4, -1), vec![1, 2, 3, 4]);
    assert_eq!(find_closest_elements(&[1, 1, 2, 3, 4, 5], 4, -1), vec![1, 1, 2, 3]);
    assert_eq!(find_closest_elements(&[1, 3], 1, 2), vec![1]);
    println!("ok");
}
