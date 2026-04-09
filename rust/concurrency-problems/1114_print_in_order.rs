// LeetCode 1114 - Print in Order
// Three threads call first(), second(), third() in arbitrary order; output must be
// "firstsecondthird".
// Approach: a Mutex<u8> stage counter plus a Condvar; each method waits for its stage.
// Time: O(1) per call (excluding waiting). Space: O(1).
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct Foo {
    stage: Mutex<u8>,
    cv: Condvar,
}

impl Foo {
    fn new() -> Self {
        Foo { stage: Mutex::new(0), cv: Condvar::new() }
    }
    fn run(&self, want: u8, f: impl FnOnce()) {
        let mut s = self.cv.wait_while(self.stage.lock().unwrap(), |s| *s != want).unwrap();
        f();
        *s += 1;
        self.cv.notify_all();
    }
    fn first(&self, f: impl FnOnce()) { self.run(0, f) }
    fn second(&self, f: impl FnOnce()) { self.run(1, f) }
    fn third(&self, f: impl FnOnce()) { self.run(2, f) }
}

fn main() {
    let foo = Arc::new(Foo::new());
    let out = Arc::new(Mutex::new(String::new()));
    let mut hs = vec![];
    for which in [2, 0, 1] {
        let (foo, out) = (foo.clone(), out.clone());
        hs.push(thread::spawn(move || match which {
            0 => foo.first(|| out.lock().unwrap().push_str("first")),
            1 => foo.second(|| out.lock().unwrap().push_str("second")),
            _ => foo.third(|| out.lock().unwrap().push_str("third")),
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    assert_eq!(*out.lock().unwrap(), "firstsecondthird");
    println!("{}", out.lock().unwrap());
}
