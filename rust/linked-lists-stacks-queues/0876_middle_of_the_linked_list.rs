// LeetCode 876 - Middle of the Linked List
// Return the middle node; with two middles, return the second one.
// Approach: slow/fast pointers over shared references (safe in Rust since we only read).
// Time O(n), Space O(1).
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn middle_node(head: &Option<Box<ListNode>>) -> Option<&ListNode> {
    let mut slow = head.as_deref();
    let mut fast = head.as_deref();
    while let Some(f) = fast.and_then(|f| f.next.as_deref()) {
        slow = slow.and_then(|s| s.next.as_deref());
        fast = f.next.as_deref();
    }
    slow
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn main() {
    assert_eq!(middle_node(&build(&[1, 2, 3, 4, 5])).unwrap().val, 3);
    assert_eq!(middle_node(&build(&[1, 2, 3, 4, 5, 6])).unwrap().val, 4);
    println!("ok");
}
