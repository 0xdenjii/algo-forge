// LeetCode 83 - Remove Duplicates from Sorted List
// Delete duplicates so each value in a sorted list appears exactly once.
// Approach: at each node, unlink following nodes that carry the same value.
// Time O(n), Space O(1).
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head.as_mut();
    while let Some(node) = cur {
        while node.next.as_ref().map_or(false, |n| n.val == node.val) {
            node.next = node.next.take().unwrap().next;
        }
        cur = node.next.as_mut();
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
    assert_eq!(to_vec(&delete_duplicates(build(&[1, 1, 2, 3, 3, 3]))), vec![1, 2, 3]);
    println!("ok");
}
