// Parallel algorithm - Parallel Merge Sort with scoped threads
// Sort a large slice using multiple cores. Recursively split, sort halves in
// parallel down to a depth limit, then merge.
// Approach: std::thread::scope lets threads borrow disjoint halves (split_at_mut)
// without Arc or 'static; below the depth limit fall back to sequential sort.
// Time: O(n log n) work, O(n) span-ish with enough cores. Space: O(n) merge buffer.
use std::thread;

fn merge<T: Ord + Copy>(a: &[T], b: &[T], out: &mut [T]) {
    let (mut i, mut j) = (0, 0);
    for slot in out.iter_mut() {
        if j >= b.len() || (i < a.len() && a[i] <= b[j]) {
            *slot = a[i];
            i += 1;
        } else {
            *slot = b[j];
            j += 1;
        }
    }
}

pub fn par_merge_sort<T: Ord + Copy + Send>(v: &mut [T], depth: u32) {
    if v.len() <= 1 {
        return;
    }
    if depth == 0 || v.len() < 4096 {
        v.sort();
        return;
    }
    let mid = v.len() / 2;
    {
        let (l, r) = v.split_at_mut(mid);
        thread::scope(|s| {
            s.spawn(|| par_merge_sort(l, depth - 1));
            par_merge_sort(r, depth - 1);
        });
    }
    let tmp = v.to_vec();
    merge(&tmp[..mid], &tmp[mid..], v);
}

fn main() {
    let mut seed: u64 = 7;
    let mut v: Vec<u32> = (0..1_000_000)
        .map(|_| {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed as u32
        })
        .collect();
    let mut expected = v.clone();
    expected.sort();
    let threads = thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    let depth = (threads as f64).log2().ceil() as u32;
    par_merge_sort(&mut v, depth);
    assert_eq!(v, expected);
    println!("sorted {} items with depth {depth}", v.len());
}
