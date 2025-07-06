// LeetCode 141/142 - Linked List Cycle I & II
// Detect whether a list has a cycle and, if so, return the node where it begins.
// Approach: Floyd tortoise-and-hare. Cyclic lists cannot be expressed with Box,
// so nodes live in an arena and `next` is an index (a common Rust idiom).
// Time O(n), Space O(1).
pub struct Arena { pub next: Vec<Option<usize>> }

impl Arena {
    fn step(&self, i: Option<usize>) -> Option<usize> { i.and_then(|i| self.next[i]) }

    pub fn has_cycle(&self, head: Option<usize>) -> bool { self.cycle_start(head).is_some() }

    pub fn cycle_start(&self, head: Option<usize>) -> Option<usize> {
        let (mut slow, mut fast) = (head, head);
        loop {
            slow = self.step(slow);
            fast = self.step(self.step(fast));
            if fast.is_none() { return None; }
            if slow == fast { break; }
        }
        let mut a = head;
        while a != slow { a = self.step(a); slow = self.step(slow); }
        a
    }
}

fn main() {
    // 0 -> 1 -> 2 -> 3 -> back to 1
    let cyc = Arena { next: vec![Some(1), Some(2), Some(3), Some(1)] };
    assert_eq!(cyc.cycle_start(Some(0)), Some(1));
    let line = Arena { next: vec![Some(1), Some(2), None] };
    assert!(!line.has_cycle(Some(0)));
    println!("ok");
}
