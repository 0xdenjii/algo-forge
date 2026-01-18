// Classic - Lexicographically Minimal String Rotation (Booth / two-pointer "minimum expression")
// Return the smallest rotation of s in lexicographic order (and the start index).
// Approach: two candidate starts i, j and match length k over s+s; on mismatch, the losing
// candidate can skip k+1 positions. Equivalent result to Booth's algorithm.
// Time: O(n), Space: O(1)

pub fn min_rotation_index(s: &[u8]) -> usize {
    let n = s.len();
    let (mut i, mut j, mut k) = (0usize, 1usize, 0usize);
    while i < n && j < n && k < n {
        let (a, b) = (s[(i + k) % n], s[(j + k) % n]);
        if a == b {
            k += 1;
            continue;
        }
        if a > b {
            i += k + 1;
        } else {
            j += k + 1;
        }
        if i == j {
            j += 1;
        }
        k = 0;
    }
    i.min(j).min(n.saturating_sub(1))
}

pub fn min_rotation(s: &str) -> String {
    let b = s.as_bytes();
    if b.is_empty() {
        return String::new();
    }
    let i = min_rotation_index(b);
    format!("{}{}", &s[i..], &s[..i])
}

fn main() {
    assert_eq!(min_rotation("bbaaccaadd"), "aaccaaddbb");
    assert_eq!(min_rotation("cabcab"), "abcabc");
    assert_eq!(min_rotation("aaaa"), "aaaa");
    println!("ok");
}
