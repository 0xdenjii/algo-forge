// Pattern - Multi-stage Pipeline
// Build a pipeline: generate numbers -> filter primes -> square -> sum, with each
// stage on its own thread, connected by bounded channels, preserving order.
// Approach: each stage owns a Receiver from the previous stage and a SyncSender to
// the next; closing propagates naturally when an upstream Sender is dropped.
// Time: O(n * sqrt(n)) for the prime filter. Space: O(channel bounds).
use std::sync::mpsc::{sync_channel, Receiver};
use std::thread;

fn stage<I, O, F>(rx: Receiver<I>, mut f: F) -> Receiver<O>
where
    I: Send + 'static,
    O: Send + 'static,
    F: FnMut(I) -> Option<O> + Send + 'static,
{
    let (tx, out) = sync_channel(16);
    thread::spawn(move || {
        for x in rx {
            if let Some(y) = f(x) {
                if tx.send(y).is_err() {
                    break;
                }
            }
        }
    });
    out
}

fn is_prime(n: u64) -> bool {
    n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| n % d != 0)
}

fn main() {
    let (tx, src) = sync_channel(16);
    thread::spawn(move || {
        for i in 0..1000u64 {
            tx.send(i).unwrap();
        }
    });
    let primes = stage(src, |x| is_prime(x).then_some(x));
    let squares = stage(primes, |x| Some(x * x));
    let collected: Vec<u64> = squares.iter().collect();
    let expected: Vec<u64> = (0..1000).filter(|&x| is_prime(x)).map(|x| x * x).collect();
    assert_eq!(collected, expected);
    println!("{} primes < 1000, sum of squares = {}", collected.len(), collected.iter().sum::<u64>());
}
