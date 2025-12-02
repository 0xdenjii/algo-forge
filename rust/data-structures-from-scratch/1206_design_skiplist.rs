// LeetCode 1206 - Design Skiplist
// Implement search(target), add(num) and erase(num) (duplicates allowed) without libraries.
// Approach: arena of nodes, each with a tower of `next` indices; node 0 is a head sentinel.
// New towers get height via coin flips from an xorshift PRNG (p = 1/2, max 16 levels).
// Erased nodes are unlinked and their slots recycled through a free list.
// Time: O(log n) expected per op. Space: O(n) expected.
const MAX_LEVEL: usize = 16;
const NIL: usize = usize::MAX;

struct Node {
    val: i32,
    next: Vec<usize>,
}

pub struct Skiplist {
    nodes: Vec<Node>,
    free: Vec<usize>,
    level: usize,
    rng: u64,
}

impl Skiplist {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node { val: i32::MIN, next: vec![NIL; MAX_LEVEL] }],
            free: Vec::new(),
            level: 1,
            rng: 0x2545_F491_4F6C_DD1D,
        }
    }

    fn random_level(&mut self) -> usize {
        let mut lvl = 1;
        loop {
            self.rng ^= self.rng << 13;
            self.rng ^= self.rng >> 7;
            self.rng ^= self.rng << 17;
            if self.rng & 1 == 0 || lvl == MAX_LEVEL {
                return lvl;
            }
            lvl += 1;
        }
    }

    /// For each level, the last node with val < target.
    fn predecessors(&self, target: i32) -> [usize; MAX_LEVEL] {
        let mut update = [0usize; MAX_LEVEL];
        let mut cur = 0;
        for l in (0..self.level).rev() {
            loop {
                let nx = self.nodes[cur].next[l];
                if nx != NIL && self.nodes[nx].val < target {
                    cur = nx;
                } else {
                    break;
                }
            }
            update[l] = cur;
        }
        update
    }

    pub fn search(&self, target: i32) -> bool {
        let p = self.predecessors(target)[0];
        let nx = self.nodes[p].next[0];
        nx != NIL && self.nodes[nx].val == target
    }

    pub fn add(&mut self, num: i32) {
        let update = self.predecessors(num);
        let lvl = self.random_level();
        if lvl > self.level {
            // update[] already holds 0 (head) for unused levels
            self.level = lvl;
        }
        let node = Node { val: num, next: vec![NIL; lvl] };
        let id = match self.free.pop() {
            Some(i) => {
                self.nodes[i] = node;
                i
            }
            None => {
                self.nodes.push(node);
                self.nodes.len() - 1
            }
        };
        for l in 0..lvl {
            let p = update[l];
            self.nodes[id].next[l] = self.nodes[p].next[l];
            self.nodes[p].next[l] = id;
        }
    }

    pub fn erase(&mut self, num: i32) -> bool {
        let update = self.predecessors(num);
        let target = self.nodes[update[0]].next[0];
        if target == NIL || self.nodes[target].val != num {
            return false;
        }
        for l in 0..self.nodes[target].next.len() {
            let p = update[l];
            if self.nodes[p].next[l] == target {
                self.nodes[p].next[l] = self.nodes[target].next[l];
            }
        }
        while self.level > 1 && self.nodes[0].next[self.level - 1] == NIL {
            self.level -= 1;
        }
        self.free.push(target);
        true
    }
}

fn main() {
    let mut s = Skiplist::new();
    s.add(1);
    s.add(2);
    s.add(3);
    assert!(!s.search(0));
    s.add(4);
    assert!(s.search(1));
    assert!(!s.erase(0));
    assert!(s.erase(1));
    assert!(!s.search(1));

    // duplicates + stress against a sorted Vec
    let mut model: Vec<i32> = Vec::new();
    let mut sl = Skiplist::new();
    for i in 0..2000i32 {
        let x = (i * 7919) % 101;
        if i % 3 == 0 {
            let removed = sl.erase(x);
            let pos = model.iter().position(|&v| v == x);
            assert_eq!(removed, pos.is_some());
            if let Some(p) = pos {
                model.remove(p);
            }
        } else {
            sl.add(x);
            model.push(x);
        }
        assert_eq!(sl.search(x), model.contains(&x));
    }
    println!("skiplist ok");
}
