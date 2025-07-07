// LeetCode 203 - Remove Linked List Elements
// Remove all nodes whose value equals `val`.
// Approach: a mutable cursor over Option slots; unlink in place or advance.
// Time O(n), Space O(1).
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut slot = &mut head;
    loop {
        match slot {
            None => break,
            Some(node) if node.val == val => *slot = node.next.take(),
            Some(node) => slot = &mut node.next,
        }
    }
    head
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn to_vec(mut l: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![]; while let Some(n) = l { out.push(n.val); l = &n.next; } out
}
fn main() {
    assert_eq!(to_vec(&remove_elements(build(&[1, 2, 6, 3, 4, 5, 6]), 6)), vec![1, 2, 3, 4, 5]);
    assert_eq!(to_vec(&remove_elements(build(&[7, 7, 7]), 7)), Vec::<i32>::new());
    println!("ok");
}
