// LeetCode 354 - Russian Doll Envelopes
// Envelope (w, h) fits into another if both width and height are strictly
// smaller. Return the maximum number of envelopes you can nest.
// Approach: sort by width ascending and height descending (so equal widths
// cannot chain), then LIS on heights with binary search.
// Time: O(n log n), Space: O(n)

fn max_envelopes(mut env: Vec<(i32, i32)>) -> usize {
    env.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    let mut tails: Vec<i32> = Vec::new();
    for &(_, h) in &env {
        let i = tails.partition_point(|&t| t < h);
        if i == tails.len() {
            tails.push(h);
        } else {
            tails[i] = h;
        }
    }
    tails.len()
}

fn main() {
    assert_eq!(max_envelopes(vec![(5, 4), (6, 4), (6, 7), (2, 3)]), 3);
    assert_eq!(max_envelopes(vec![(1, 1), (1, 1), (1, 1)]), 1);
    println!("ok");
}
