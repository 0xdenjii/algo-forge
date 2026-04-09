// LeetCode 1116 - Print Zero Even Odd
// Three threads: zero() prints 0s, even() prints even numbers, odd() prints odd numbers.
// For n the combined output must be "0102...0n".
// Approach: state machine (next number + whether a zero is due) under a Mutex;
// all threads wait on one Condvar for their condition.
// Time: O(n). Space: O(1).
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct State {
    i: usize,
    zero_due: bool,
}

struct ZeroEvenOdd {
    n: usize,
    st: Mutex<State>,
    cv: Condvar,
}

impl ZeroEvenOdd {
    fn new(n: usize) -> Self {
        ZeroEvenOdd { n, st: Mutex::new(State { i: 1, zero_due: true }), cv: Condvar::new() }
    }
    fn zero(&self, mut p: impl FnMut(usize)) {
        for _ in 0..self.n {
            let mut s = self.cv.wait_while(self.st.lock().unwrap(), |s| !s.zero_due).unwrap();
            p(0);
            s.zero_due = false;
            self.cv.notify_all();
        }
    }
    fn numbers(&self, parity: usize, mut p: impl FnMut(usize)) {
        let count = (1..=self.n).filter(|x| x % 2 == parity).count();
        for _ in 0..count {
            let mut s = self
                .cv
                .wait_while(self.st.lock().unwrap(), |s| s.zero_due || s.i % 2 != parity)
                .unwrap();
            p(s.i);
            s.i += 1;
            s.zero_due = true;
            self.cv.notify_all();
        }
    }
    fn even(&self, p: impl FnMut(usize)) { self.numbers(0, p) }
    fn odd(&self, p: impl FnMut(usize)) { self.numbers(1, p) }
}

fn main() {
    let z = Arc::new(ZeroEvenOdd::new(5));
    let out = Arc::new(Mutex::new(String::new()));
    let mk = |o: &Arc<Mutex<String>>| {
        let o = o.clone();
        move |x: usize| o.lock().unwrap().push_str(&x.to_string())
    };
    let (a, b, c) = (z.clone(), z.clone(), z.clone());
    let (pa, pb, pc) = (mk(&out), mk(&out), mk(&out));
    let hs = vec![
        thread::spawn(move || a.zero(pa)),
        thread::spawn(move || b.even(pb)),
        thread::spawn(move || c.odd(pc)),
    ];
    for h in hs {
        h.join().unwrap();
    }
    assert_eq!(*out.lock().unwrap(), "0102030405");
    println!("{}", out.lock().unwrap());
}
