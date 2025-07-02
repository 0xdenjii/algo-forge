// LeetCode 92 - Reverse Linked List II
// Reverse the nodes from position left to right (1-indexed, inclusive) in one pass.
// Approach: move a cursor to the slot before `left`, detach the segment while
// reversing it, then reattach the remainder to the segment's new tail.
// Time O(n), Space O(1).
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn reverse_between(mut head: Option<Box<ListNode>>, left: usize, right: usize) -> Option<Box<ListNode>> {
    let mut slot = &mut head;
    for _ in 1..left { slot = &mut slot.as_mut().unwrap().next; }
    let mut rest = slot.take();
    let mut rev: Option<Box<ListNode>> = None;
    for _ in left..=right {
        let mut node = rest.take().unwrap();
        rest = node.next.take();
        node.next = rev;
        rev = Some(node);
    }
    *slot = rev;
    let mut tail = slot;
    while tail.is_some() { tail = &mut tail.as_mut().unwrap().next; }
    *tail = rest;
    head
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn to_vec(mut l: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![]; while let Some(n) = l { out.push(n.val); l = &n.next; } out
}
fn main() {
    assert_eq!(to_vec(&reverse_between(build(&[1, 2, 3, 4, 5]), 2, 4)), vec![1, 4, 3, 2, 5]);
    assert_eq!(to_vec(&reverse_between(build(&[5]), 1, 1)), vec![5]);
    println!("ok");
}
