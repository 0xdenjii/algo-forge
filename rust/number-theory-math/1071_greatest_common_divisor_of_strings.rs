// LeetCode 1071 - Greatest Common Divisor of Strings
// Return the largest string x such that both s and t are concatenations of copies of x.
// Approach: a common divisor exists iff s + t == t + s; then the answer is the prefix of
// length gcd(|s|, |t|).
// Time: O(m + n), Space: O(m + n)
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn gcd_of_strings(s: &str, t: &str) -> String {
    if format!("{s}{t}") != format!("{t}{s}") {
        return String::new();
    }
    s[..gcd(s.len(), t.len())].to_string()
}

fn main() {
    assert_eq!(gcd_of_strings("ABCABC", "ABC"), "ABC");
    assert_eq!(gcd_of_strings("ABABAB", "ABAB"), "AB");
    assert_eq!(gcd_of_strings("LEET", "CODE"), "");
    println!("ok");
}
