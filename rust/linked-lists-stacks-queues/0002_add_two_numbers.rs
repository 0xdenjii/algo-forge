// LeetCode 2 - Add Two Numbers
// Two non-empty lists store non-negative integers in reverse digit order.
// Return their sum as a list in the same format.
// Approach: simulate grade-school addition with a carry.
// Time O(max(n, m)), Space O(max(n, m)) for the output.
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn add_two_numbers(mut a: Option<Box<ListNode>>, mut b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;
    let mut carry = 0;
    while a.is_some() || b.is_some() || carry > 0 {
        let mut sum = carry;
        if let Some(n) = a { sum += n.val; a = n.next; }
        if let Some(n) = b { sum += n.val; b = n.next; }
        carry = sum / 10;
        tail = &mut tail.insert(Box::new(ListNode { val: sum % 10, next: None })).next;
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
    assert_eq!(to_vec(&add_two_numbers(build(&[2, 4, 3]), build(&[5, 6, 4]))), vec![7, 0, 8]);
    assert_eq!(to_vec(&add_two_numbers(build(&[9, 9, 9]), build(&[1]))), vec![0, 0, 0, 1]);
    println!("ok");
}
