// Title: Mo's Algorithm (SPOJ DQUERY - distinct elements in a range)
// Problem: Static array of n values; answer q offline queries: how many distinct values are
// in a[l..=r]?
// Approach: Sort queries by (l / block, r ascending or descending by block parity), then move
// a half-open window [cl, cr) maintaining value counts and the distinct count.
// Complexity: O((n + q) * sqrt(n)) time, O(n + q + maxV) space.
pub fn distinct_queries(a: &[usize], qs: &[(usize, usize)]) -> Vec<usize> {
    let blk = ((a.len() as f64).sqrt() as usize).max(1);
    let mut ord: Vec<usize> = (0..qs.len()).collect();
    ord.sort_by_key(|&i| {
        let (l, r) = qs[i];
        let b = l / blk;
        (b, if b % 2 == 0 { r } else { usize::MAX - r })
    });
    let mut cnt = vec![0u32; a.iter().max().map_or(1, |m| m + 1)];
    let (mut cl, mut cr, mut distinct) = (0usize, 0usize, 0usize);
    let mut ans = vec![0; qs.len()];
    for i in ord {
        let (l, r) = (qs[i].0, qs[i].1 + 1);
        while cr < r {
            cnt[a[cr]] += 1;
            if cnt[a[cr]] == 1 {
                distinct += 1;
            }
            cr += 1;
        }
        while cl > l {
            cl -= 1;
            cnt[a[cl]] += 1;
            if cnt[a[cl]] == 1 {
                distinct += 1;
            }
        }
        while cr > r {
            cr -= 1;
            cnt[a[cr]] -= 1;
            if cnt[a[cr]] == 0 {
                distinct -= 1;
            }
        }
        while cl < l {
            cnt[a[cl]] -= 1;
            if cnt[a[cl]] == 0 {
                distinct -= 1;
            }
            cl += 1;
        }
        ans[i] = distinct;
    }
    ans
}

fn main() {
    assert_eq!(distinct_queries(&[1, 1, 2, 1, 3], &[(0, 4), (1, 3), (2, 4)]), vec![3, 2, 3]);
    assert_eq!(distinct_queries(&[5, 5, 5], &[(0, 2), (1, 1)]), vec![1, 1]);
    println!("ok");
}
