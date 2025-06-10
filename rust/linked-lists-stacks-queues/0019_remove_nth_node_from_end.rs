// LeetCode 19 - Remove Nth Node From End of List
// Remove the n-th node from the end of a list and return its head.
// Approach: count the length, walk a mutable cursor to index len-n and unlink.
// (The two-pointer variant needs aliasing; counting is the idiomatic safe-Rust form.)
// Time O(n), Space O(1).
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: usize) -> Option<Box<ListNode>> {
    let mut len = 0;
    let mut p = &head;
    while let Some(node) = p { len += 1; p = &node.next; }
    let mut cur = &mut head;
    for _ in 0..len - n { cur = &mut cur.as_mut().unwrap().next; }
    let removed = cur.take().unwrap();
    *cur = removed.next;
    head
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn to_vec(mut l: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![]; while let Some(n) = l { out.push(n.val); l = &n.next; } out
}
fn main() {
    assert_eq!(to_vec(&remove_nth_from_end(build(&[1, 2, 3, 4, 5]), 2)), vec![1, 2, 3, 5]);
    assert_eq!(to_vec(&remove_nth_from_end(build(&[1]), 1)), Vec::<i32>::new());
    println!("ok");
}
