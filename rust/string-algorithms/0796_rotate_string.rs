// LeetCode 796 - Rotate String
// Return true if goal can be obtained from s by some number of left rotations.
// Approach: goal is a rotation of s iff lengths match and goal occurs in s + s
// (searched with the Z-function on goal + '\0' + s + s).
// Time: O(n), Space: O(n)

fn z_function(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i < r {
            z[i] = z[i - l].min(r - i);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}

pub fn rotate_string(s: &str, goal: &str) -> bool {
    if s.len() != goal.len() {
        return false;
    }
    if s.is_empty() {
        return true;
    }
    let mut buf = goal.as_bytes().to_vec();
    buf.push(0);
    buf.extend_from_slice(s.as_bytes());
    buf.extend_from_slice(s.as_bytes());
    z_function(&buf).iter().skip(goal.len() + 1).any(|&v| v >= goal.len())
}

fn main() {
    assert!(rotate_string("abcde", "cdeab"));
    assert!(!rotate_string("abcde", "abced"));
    println!("ok");
}
