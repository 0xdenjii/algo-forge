// LeetCode 20 - Valid Parentheses
// Given a string of '()[]{}', decide whether every bracket is closed in the right order.
// Approach: push the expected closing bracket for each opener; pop and compare on closers.
// Time O(n), Space O(n).
pub fn is_valid(s: &str) -> bool {
    let mut stack = Vec::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            _ => if stack.pop() != Some(c) { return false; },
        }
    }
    stack.is_empty()
}

fn main() {
    assert!(is_valid("()[]{}"));
    assert!(is_valid("{[()]}"));
    assert!(!is_valid("(]"));
    assert!(!is_valid("(("));
    println!("ok");
}
