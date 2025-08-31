// LeetCode 275 - H-Index II
// citations is sorted ascending. Return the researcher's h-index: the largest
// h such that h papers have at least h citations each. O(log n) required.
// Approach: find the first i with citations[i] >= n - i; answer is n - i.
// Time: O(log n), Space: O(1)

fn h_index(citations: &[i32]) -> usize {
    let n = citations.len();
    let (mut lo, mut hi) = (0, n);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if (citations[mid] as usize) < n - mid { lo = mid + 1 } else { hi = mid }
    }
    n - lo
}

fn main() {
    assert_eq!(h_index(&[0, 1, 3, 5, 6]), 3);
    assert_eq!(h_index(&[1, 2, 100]), 2);
    assert_eq!(h_index(&[0]), 0);
    assert_eq!(h_index(&[100]), 1);
    println!("ok");
}
