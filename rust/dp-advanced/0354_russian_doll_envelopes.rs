// LeetCode 354 - Russian Doll Envelopes
// Envelope (w, h) fits into another if both dimensions are strictly smaller.
// Return the maximum number of envelopes that can be nested.
// Approach: sort by width asc and height desc (so equal widths cannot chain),
// then run O(n log n) LIS on heights.
// Time: O(n log n), Space: O(n)

pub fn max_envelopes(mut env: Vec<(i32, i32)>) -> usize {
    env.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    let mut tails: Vec<i32> = Vec::new();
    for (_, h) in env {
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
