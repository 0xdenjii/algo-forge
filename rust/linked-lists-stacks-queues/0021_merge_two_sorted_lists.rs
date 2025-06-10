// LeetCode 21 - Merge Two Sorted Lists
// Merge two sorted singly linked lists into one sorted list by splicing nodes.
// Approach: keep a mutable cursor at the output tail slot, always taking the smaller head.
// Time O(n + m), Space O(1).
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn merge_two_lists(mut a: Option<Box<ListNode>>, mut b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;
    loop {
        let take_a = match (&a, &b) {
            (Some(x), Some(y)) => x.val <= y.val,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => break,
        };
        let src = if take_a { &mut a } else { &mut b };
        let mut node = src.take().unwrap();
        *src = node.next.take();
        tail = &mut tail.insert(node).next;
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
    let m = merge_two_lists(build(&[1, 2, 4]), build(&[1, 3, 4]));
    assert_eq!(to_vec(&m), vec![1, 1, 2, 3, 4, 4]);
    println!("ok");
}
