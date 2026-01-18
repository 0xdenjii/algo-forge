// Classic - Suffix Automaton (DAWG)
// Build the minimal automaton accepting all substrings of s, then answer:
// number of distinct substrings, substring membership, and longest common substring with t.
// Approach: online construction with clone states; distinct = sum(len[v] - len[link[v]]).
// Time: O(n * SIGMA) build with array transitions, Space: O(n * SIGMA)

pub struct SuffixAutomaton {
    next: Vec<[i32; 26]>,
    link: Vec<i32>,
    len: Vec<usize>,
    last: usize,
}

impl SuffixAutomaton {
    pub fn new(s: &str) -> Self {
        let mut sam = SuffixAutomaton { next: vec![[-1; 26]], link: vec![-1], len: vec![0], last: 0 };
        for b in s.bytes() {
            sam.extend((b - b'a') as usize);
        }
        sam
    }

    fn new_state(&mut self, len: usize, next: [i32; 26], link: i32) -> usize {
        self.next.push(next);
        self.link.push(link);
        self.len.push(len);
        self.next.len() - 1
    }

    fn extend(&mut self, c: usize) {
        let cur = self.new_state(self.len[self.last] + 1, [-1; 26], 0);
        let mut p = self.last as i32;
        while p != -1 && self.next[p as usize][c] == -1 {
            self.next[p as usize][c] = cur as i32;
            p = self.link[p as usize];
        }
        if p != -1 {
            let q = self.next[p as usize][c] as usize;
            if self.len[p as usize] + 1 == self.len[q] {
                self.link[cur] = q as i32;
            } else {
                let clone = self.new_state(self.len[p as usize] + 1, self.next[q], self.link[q]);
                while p != -1 && self.next[p as usize][c] == q as i32 {
                    self.next[p as usize][c] = clone as i32;
                    p = self.link[p as usize];
                }
                self.link[q] = clone as i32;
                self.link[cur] = clone as i32;
            }
        }
        self.last = cur;
    }

    pub fn distinct_substrings(&self) -> usize {
        (1..self.len.len()).map(|v| self.len[v] - self.len[self.link[v] as usize]).sum()
    }

    pub fn contains(&self, t: &str) -> bool {
        let mut v = 0usize;
        for b in t.bytes() {
            match self.next[v][(b - b'a') as usize] {
                -1 => return false,
                u => v = u as usize,
            }
        }
        true
    }

    /// Length of the longest common substring between the indexed string and t.
    pub fn lcs_len(&self, t: &str) -> usize {
        let (mut v, mut l, mut best) = (0usize, 0usize, 0usize);
        for b in t.bytes() {
            let c = (b - b'a') as usize;
            while v != 0 && self.next[v][c] == -1 {
                v = self.link[v] as usize;
                l = self.len[v];
            }
            if self.next[v][c] != -1 {
                v = self.next[v][c] as usize;
                l += 1;
            }
            best = best.max(l);
        }
        best
    }
}

fn main() {
    let sam = SuffixAutomaton::new("banana");
    assert_eq!(sam.distinct_substrings(), 15);
    assert!(sam.contains("nan"));
    assert!(!sam.contains("nab"));
    assert_eq!(sam.lcs_len("xxanaxx"), 3);
    println!("ok");
}
