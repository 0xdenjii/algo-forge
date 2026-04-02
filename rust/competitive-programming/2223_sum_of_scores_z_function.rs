// Problem 2223: Sum of Scores of Built Strings
// The string s is built by prepending characters; s_i is the suffix of length i. The score of
// s_i is the length of its longest common prefix with s. Return the sum of all scores.
// Approach: Z-function: z[i] = LCP(s, s[i..]); with z[0] = n the answer is the sum of z.
// Complexity: O(n) time and space.
pub fn z_function(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i < r {
            z[i] = (r - i).min(z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    if n > 0 {
        z[0] = n;
    }
    z
}

pub fn sum_scores(s: &str) -> u64 {
    z_function(s.as_bytes()).iter().map(|&x| x as u64).sum()
}

fn main() {
    assert_eq!(sum_scores("babab"), 9);
    assert_eq!(sum_scores("azbazbzaz"), 14);
    assert_eq!(z_function(b"aaaaa"), vec![5, 4, 3, 2, 1]);
    println!("ok");
}
