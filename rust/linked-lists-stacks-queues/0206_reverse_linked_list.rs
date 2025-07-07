// LeetCode 206 - Reverse Linked List
// Given the head of a singly linked list, reverse it and return the new head.
// Approach: iteratively detach each node and push it onto a new list.
// Time O(n), Space O(1).
#[derive(Debug, PartialEq)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn to_vec(mut l: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![]; while let Some(n) = l { out.push(n.val); l = &n.next; } out
}
fn main() {
    assert_eq!(to_vec(&reverse_list(build(&[1, 2, 3, 4, 5]))), vec![5, 4, 3, 2, 1]);
    assert_eq!(to_vec(&reverse_list(None)), Vec::<i32>::new());
    println!("ok");
}
