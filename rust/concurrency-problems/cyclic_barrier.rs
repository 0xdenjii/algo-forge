// Synchronizer - Reusable Cyclic Barrier
// N threads call wait(); all block until the N-th arrives, then all proceed. The
// barrier must be reusable across phases without a fast thread lapping slow ones.
// Approach: generation counter. Arrivals increment `count`; the last one resets it,
// bumps `generation` and notifies. Waiters sleep until the generation changes.
// Time: O(1) per wait. Space: O(1).
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub struct CyclicBarrier {
    n: usize,
    st: Mutex<(usize, u64)>, // (arrived, generation)
    cv: Condvar,
}

impl CyclicBarrier {
    pub fn new(n: usize) -> Self {
        Self { n, st: Mutex::new((0, 0)), cv: Condvar::new() }
    }
    /// Returns true for exactly one "leader" thread per phase.
    pub fn wait(&self) -> bool {
        let mut s = self.st.lock().unwrap();
        let gen = s.1;
        s.0 += 1;
        if s.0 == self.n {
            s.0 = 0;
            s.1 += 1;
            self.cv.notify_all();
            true
        } else {
            let _s = self.cv.wait_while(s, |s| s.1 == gen).unwrap();
            false
        }
    }
}

fn main() {
    const N: usize = 4;
    const PHASES: usize = 5;
    let b = Arc::new(CyclicBarrier::new(N));
    let log = Arc::new(Mutex::new(Vec::new()));
    let hs: Vec<_> = (0..N)
        .map(|id| {
            let (b, log) = (b.clone(), log.clone());
            thread::spawn(move || {
                for phase in 0..PHASES {
                    log.lock().unwrap().push((phase, id));
                    b.wait();
                }
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    let log = log.lock().unwrap();
    // every entry of phase p must appear before any entry of phase p+1
    for w in log.windows(2) {
        assert!(w[0].0 <= w[1].0);
    }
    println!("{} entries, phases strictly ordered", log.len());
}
