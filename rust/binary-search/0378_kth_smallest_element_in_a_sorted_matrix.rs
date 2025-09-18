// LeetCode 378 - Kth Smallest Element in a Sorted Matrix
// In an n x n matrix whose rows and columns are sorted ascending, return the
// kth smallest element (1-based, counting duplicates).
// Approach: binary search on value; count elements <= v with a staircase walk
// from the bottom-left corner in O(n).
// Time: O(n log(max - min)), Space: O(1)

fn count_le(m: &[Vec<i32>], v: i32) -> usize {
    let n = m.len();
    let (mut row, mut col, mut cnt) = (n as isize - 1, 0usize, 0usize);
    while row >= 0 && col < n {
        if m[row as usize][col] <= v {
            cnt += row as usize + 1;
            col += 1;
        } else {
            row -= 1;
        }
    }
    cnt
}

fn kth_smallest(m: &[Vec<i32>], k: usize) -> i32 {
    let n = m.len();
    let (mut lo, mut hi) = (m[0][0] as i64, m[n - 1][n - 1] as i64);
    while lo < hi {
        let mid = lo + (hi - lo).div_euclid(2);
        if count_le(m, mid as i32) >= k { hi = mid } else { lo = mid + 1 }
    }
    lo as i32
}

fn main() {
    let m = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
    assert_eq!(kth_smallest(&m, 8), 13);
    assert_eq!(kth_smallest(&[vec![-5]], 1), -5);
    assert_eq!(kth_smallest(&[vec![-5, -4], vec![-5, -4]], 2), -5);
    println!("ok");
}
