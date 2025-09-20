// LeetCode 1095 - Find in Mountain Array
// Given a mountain array accessible only through get(i) and len(), return the
// minimum index with value target, or -1. At most ~100 get calls allowed.
// Approach: binary search the peak, then search the ascending left part, then
// the descending right part.
// Time: O(log n) get calls, Space: O(1)

trait MountainArray {
    fn get(&self, i: usize) -> i32;
    fn len(&self) -> usize;
}

impl MountainArray for Vec<i32> {
    fn get(&self, i: usize) -> i32 {
        self[i]
    }
    fn len(&self) -> usize {
        Vec::len(self)
    }
}

fn find_in_mountain_array(target: i32, arr: &impl MountainArray) -> i32 {
    let n = arr.len();
    let (mut lo, mut hi) = (0, n - 1);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if arr.get(mid) < arr.get(mid + 1) { lo = mid + 1 } else { hi = mid }
    }
    let peak = lo;

    let search = |mut lo: usize, mut hi: usize, ascending: bool| -> Option<usize> {
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let v = arr.get(mid);
            let go_right = if ascending { v < target } else { v > target };
            if go_right { lo = mid + 1 } else { hi = mid }
        }
        (lo < n && arr.get(lo) == target).then_some(lo)
    };

    search(0, peak, true)
        .or_else(|| search(peak, n - 1, false))
        .map_or(-1, |i| i as i32)
}

fn main() {
    assert_eq!(find_in_mountain_array(3, &vec![1, 2, 3, 4, 5, 3, 1]), 2);
    assert_eq!(find_in_mountain_array(3, &vec![0, 1, 2, 4, 2, 1]), -1);
    assert_eq!(find_in_mountain_array(1, &vec![0, 5, 3, 1]), 3);
    assert_eq!(find_in_mountain_array(5, &vec![0, 5, 3, 1]), 1);
    println!("ok");
}
