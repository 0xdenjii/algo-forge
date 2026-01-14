// LeetCode 72 - Edit Distance (Levenshtein)
// Minimum number of insert/delete/replace operations to convert word a into word b.
// Approach: DP over prefixes with a single rolling row plus the saved diagonal.
// Time: O(m*n), Space: O(n)

pub fn min_distance(a: &str, b: &str) -> usize {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let mut row: Vec<usize> = (0..=b.len()).collect();
    for i in 1..=a.len() {
        let mut diag = row[0];
        row[0] = i;
        for j in 1..=b.len() {
            let tmp = row[j];
            row[j] = if a[i - 1] == b[j - 1] {
                diag
            } else {
                1 + diag.min(row[j]).min(row[j - 1])
            };
            diag = tmp;
        }
    }
    row[b.len()]
}

fn main() {
    assert_eq!(min_distance("horse", "ros"), 3);
    assert_eq!(min_distance("intention", "execution"), 5);
    assert_eq!(min_distance("", "abc"), 3);
    println!("ok");
}
