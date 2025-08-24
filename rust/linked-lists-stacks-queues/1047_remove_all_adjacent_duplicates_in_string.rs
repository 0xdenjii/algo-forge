// LeetCode 1047 - Remove All Adjacent Duplicates In String
// Repeatedly remove pairs of adjacent equal letters until none remain.
// Approach: use the output String itself as a stack.
// Time O(n), Space O(n).
pub fn remove_duplicates(s: &str) -> String {
    let mut st = String::with_capacity(s.len());
    for c in s.chars() {
        if st.ends_with(c) { st.pop(); } else { st.push(c); }
    }
    st
}

fn main() {
    assert_eq!(remove_duplicates("abbaca"), "ca");
    assert_eq!(remove_duplicates("azxxzy"), "ay");
    println!("ok");
}
