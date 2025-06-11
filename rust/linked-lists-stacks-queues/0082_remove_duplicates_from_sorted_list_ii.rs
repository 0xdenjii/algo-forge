// LeetCode 82 - Remove Duplicates from Sorted List II
// Delete every node whose value occurs more than once, keeping only distinct values.
// Approach: consume the input run by run; keep a node only if its run has length 1.
// Time O(n), Space O(1).
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut out = None;
    let mut tail = &mut out;
    while let Some(mut node) = head {
        head = node.next.take();
        let mut dup = false;
        while head.as_ref().map_or(false, |n| n.val == node.val) {
            dup = true;
            head = head.unwrap().next;
        }
        if !dup { tail = &mut tail.insert(node).next; }
    }
    out
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn to_vec(mut l: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![]; while let Some(n) = l { out.push(n.val); l = &n.next; } out
}
fn main() {
    assert_eq!(to_vec(&delete_duplicates(build(&[1, 2, 3, 3, 4, 4, 5]))), vec![1, 2, 5]);
    assert_eq!(to_vec(&delete_duplicates(build(&[1, 1, 1, 2, 3]))), vec![2, 3]);
    println!("ok");
}
