// Classic - Z-Algorithm (Z-function) pattern search
// z[i] is the length of the longest common prefix of s and s[i..].
// Use it to find every occurrence of a pattern in a text via pattern + sentinel + text.
// Approach: maintain the rightmost Z-box [l, r) and reuse values inside it.
// Time: O(n + m), Space: O(n + m)

pub fn z_function<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    if n > 0 {
        z[0] = n;
    }
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

/// All start positions of `pat` in `text`. Uses Option<u8> so no sentinel byte can collide.
pub fn find_all(text: &str, pat: &str) -> Vec<usize> {
    if pat.is_empty() {
        return (0..=text.len()).collect();
    }
    let seq: Vec<Option<u8>> = pat
        .bytes()
        .map(Some)
        .chain(std::iter::once(None))
        .chain(text.bytes().map(Some))
        .collect();
    let m = pat.len();
    z_function(&seq)
        .iter()
        .enumerate()
        .skip(m + 1)
        .filter(|&(_, &v)| v >= m)
        .map(|(i, _)| i - m - 1)
        .collect()
}

fn main() {
    assert_eq!(z_function(b"aaabaab"), vec![7, 2, 1, 0, 2, 1, 0]);
    assert_eq!(find_all("abracadabra", "abra"), vec![0, 7]);
    assert_eq!(find_all("aaaa", "aa"), vec![0, 1, 2]);
    println!("ok");
}
