// LeetCode 707 - Design Linked List
// Singly linked list with get(index), add_at_head, add_at_tail, add_at_index, delete_at_index.
// Invalid indices are ignored (get returns -1).
// Approach: Option<Box<Node>> chain; walk a mutable cursor to the link at `index` and splice.
// Time: O(index) per op. Space: O(n).
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

#[derive(Default)]
pub struct MyLinkedList {
    head: Option<Box<Node>>,
    len: usize,
}

impl MyLinkedList {
    pub fn new() -> Self {
        Self::default()
    }

    fn link_at(&mut self, index: usize) -> &mut Option<Box<Node>> {
        let mut cur = &mut self.head;
        for _ in 0..index {
            cur = &mut cur.as_mut().unwrap().next;
        }
        cur
    }

    pub fn get(&self, index: i32) -> i32 {
        if index < 0 || index as usize >= self.len {
            return -1;
        }
        let mut cur = self.head.as_deref();
        for _ in 0..index {
            cur = cur.and_then(|n| n.next.as_deref());
        }
        cur.map_or(-1, |n| n.val)
    }

    pub fn add_at_head(&mut self, val: i32) {
        self.add_at_index(0, val);
    }

    pub fn add_at_tail(&mut self, val: i32) {
        self.add_at_index(self.len as i32, val);
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 || index as usize > self.len {
            return;
        }
        let link = self.link_at(index as usize);
        let next = link.take();
        *link = Some(Box::new(Node { val, next }));
        self.len += 1;
    }

    pub fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index as usize >= self.len {
            return;
        }
        let link = self.link_at(index as usize);
        let removed = link.take().unwrap();
        *link = removed.next;
        self.len -= 1;
    }
}

impl Drop for MyLinkedList {
    // Iterative drop avoids stack overflow on very long lists.
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut n) = cur {
            cur = n.next.take();
        }
    }
}

fn main() {
    let mut l = MyLinkedList::new();
    l.add_at_head(1);
    l.add_at_tail(3);
    l.add_at_index(1, 2);
    assert_eq!(l.get(1), 2);
    l.delete_at_index(1);
    assert_eq!(l.get(1), 3);
    assert_eq!(l.get(5), -1);
    println!("linked list ok");
}
