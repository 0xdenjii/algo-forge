// LeetCode 1226 - The Dining Philosophers
// Five philosophers share five forks; each needs both neighbouring forks to eat.
// Implement wantsToEat so the system never deadlocks.
// Approach: resource ordering - always lock the lower-numbered fork first, which
// breaks the circular-wait condition (Coffman).
// Time: O(1) per meal. Space: O(n) forks.
use std::sync::{Arc, Mutex};
use std::thread;

struct DiningPhilosophers {
    forks: Vec<Mutex<()>>,
}

impl DiningPhilosophers {
    fn new(n: usize) -> Self {
        Self { forks: (0..n).map(|_| Mutex::new(())).collect() }
    }
    fn wants_to_eat(&self, p: usize, eat: impl FnOnce()) {
        let n = self.forks.len();
        let (l, r) = (p, (p + 1) % n);
        let (a, b) = (l.min(r), l.max(r));
        let _g1 = self.forks[a].lock().unwrap();
        let _g2 = self.forks[b].lock().unwrap();
        eat();
    }
}

fn main() {
    let dp = Arc::new(DiningPhilosophers::new(5));
    let meals = Arc::new(Mutex::new(vec![0; 5]));
    let hs: Vec<_> = (0..5)
        .map(|p| {
            let (dp, meals) = (dp.clone(), meals.clone());
            thread::spawn(move || {
                for _ in 0..100 {
                    dp.wants_to_eat(p, || meals.lock().unwrap()[p] += 1)
                }
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    assert!(meals.lock().unwrap().iter().all(|&m| m == 100));
    println!("meals: {:?}", meals.lock().unwrap());
}
