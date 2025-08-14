// LeetCode 394 - Decode String
// Decode strings like "3[a2[c]]" -> "accaccacc" (k[encoded] repeats encoded k times).
// Approach: on '[' push (current string, repeat count) and start fresh; on ']'
// pop and append current repeated k times to the saved prefix.
// Time O(output length), Space O(output length).
pub fn decode_string(s: &str) -> String {
    let mut st: Vec<(String, usize)> = Vec::new();
    let mut cur = String::new();
    let mut k = 0usize;
    for c in s.chars() {
        match c {
            '0'..='9' => k = k * 10 + c.to_digit(10).unwrap() as usize,
            '[' => { st.push((std::mem::take(&mut cur), k)); k = 0; }
            ']' => {
                let (mut prev, times) = st.pop().expect("unbalanced brackets");
                prev.push_str(&cur.repeat(times));
                cur = prev;
            }
            _ => cur.push(c),
        }
    }
    cur
}

fn main() {
    assert_eq!(decode_string("3[a]2[bc]"), "aaabcbc");
    assert_eq!(decode_string("3[a2[c]]"), "accaccacc");
    assert_eq!(decode_string("2[abc]3[cd]ef"), "abcabccdcdcdef");
    assert_eq!(decode_string("10[x]"), "xxxxxxxxxx");
    println!("ok");
}
