// LeetCode 234 - Palindrome Linked List
// Return true if the list reads the same forwards and backwards.
// Approach: detach and reverse the second half, compare against the first half.
// Time O(n), Space O(1) (list is consumed; pass a clone if you must keep it).
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
    let mut n = 0;
    let mut p = &head;
    while let Some(node) = p { n += 1; p = &node.next; }
    let mut slot = &mut head;
    for _ in 0..(n + 1) / 2 { slot = &mut slot.as_mut().unwrap().next; }
    let mut second = slot.take();
    let mut rev: Option<Box<ListNode>> = None;
    while let Some(mut node) = second { second = node.next.take(); node.next = rev; rev = Some(node); }
    let (mut a, mut b) = (&head, &rev);
    while let (Some(x), Some(y)) = (a, b) {
        if x.val != y.val { return false; }
        a = &x.next;
        b = &y.next;
    }
    true
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn main() {
    assert!(is_palindrome(build(&[1, 2, 2, 1])));
    assert!(is_palindrome(build(&[1, 2, 3, 2, 1])));
    assert!(!is_palindrome(build(&[1, 2])));
    println!("ok");
}
