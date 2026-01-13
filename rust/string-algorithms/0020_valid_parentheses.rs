// LeetCode 20 - Valid Parentheses
// Given a string of brackets ()[]{}, decide whether they are closed in the correct order.
// Approach: stack of expected closing brackets.
// Time: O(n), Space: O(n)

pub fn is_valid(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            _ => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}

fn main() {
    assert!(is_valid("()[]{}"));
    assert!(is_valid("{[]}"));
    assert!(!is_valid("(]"));
    assert!(!is_valid("(("));
    println!("ok");
}
