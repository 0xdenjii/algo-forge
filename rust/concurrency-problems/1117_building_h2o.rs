// LeetCode 1117 - Building H2O
// Hydrogen and oxygen threads arrive in any order; they must be released in groups
// of exactly two H and one O (each consecutive triple of output forms a molecule).
// Approach: Mutex-protected free-slot counters for the current molecule; when all
// three slots are consumed they reset. A Condvar wakes blocked atoms.
// Time: O(1) per atom. Space: O(1).
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct Slots {
    h: u8,
    o: u8,
}

struct H2O {
    s: Mutex<Slots>,
    cv: Condvar,
}

impl H2O {
    fn new() -> Self {
        H2O { s: Mutex::new(Slots { h: 2, o: 1 }), cv: Condvar::new() }
    }
    fn take(&self, is_h: bool, release: impl FnOnce()) {
        let mut s = self
            .cv
            .wait_while(self.s.lock().unwrap(), |s| if is_h { s.h == 0 } else { s.o == 0 })
            .unwrap();
        if is_h { s.h -= 1 } else { s.o -= 1 }
        release();
        if s.h == 0 && s.o == 0 {
            s.h = 2;
            s.o = 1;
        }
        self.cv.notify_all();
    }
    fn hydrogen(&self, r: impl FnOnce()) { self.take(true, r) }
    fn oxygen(&self, r: impl FnOnce()) { self.take(false, r) }
}

fn main() {
    let input = "OOHHHHHOH";
    let w = Arc::new(H2O::new());
    let out = Arc::new(Mutex::new(String::new()));
    let hs: Vec<_> = input
        .chars()
        .map(|c| {
            let (w, out) = (w.clone(), out.clone());
            thread::spawn(move || {
                if c == 'H' {
                    w.hydrogen(|| out.lock().unwrap().push('H'))
                } else {
                    w.oxygen(|| out.lock().unwrap().push('O'))
                }
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    let s = out.lock().unwrap().clone();
    for chunk in s.as_bytes().chunks(3) {
        assert_eq!(chunk.iter().filter(|&&b| b == b'H').count(), 2);
    }
    println!("{s}");
}
