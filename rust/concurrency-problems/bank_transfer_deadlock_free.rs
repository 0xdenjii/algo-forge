// Classic - Deadlock-free Bank Transfers
// Many threads transfer money between random pairs of accounts, each guarded by its
// own lock. Naively locking (from, to) deadlocks when A->B and B->A run together.
// Total money must be conserved.
// Approach: global lock ordering by account id - always lock the smaller id first.
// Overdrafts are rejected with a Result.
// Time: O(1) per transfer. Space: O(accounts).
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, PartialEq)]
pub enum TransferError { SameAccount, Insufficient }

pub struct Bank { accounts: Vec<Mutex<i64>> }

impl Bank {
    pub fn new(balances: &[i64]) -> Self {
        Self { accounts: balances.iter().map(|&b| Mutex::new(b)).collect() }
    }
    pub fn transfer(&self, from: usize, to: usize, amt: i64) -> Result<(), TransferError> {
        if from == to { return Err(TransferError::SameAccount); }
        let (lo, hi) = (from.min(to), from.max(to));
        let mut g_lo = self.accounts[lo].lock().unwrap();
        let mut g_hi = self.accounts[hi].lock().unwrap();
        let (src, dst) = if from == lo { (&mut *g_lo, &mut *g_hi) } else { (&mut *g_hi, &mut *g_lo) };
        if *src < amt { return Err(TransferError::Insufficient); }
        *src -= amt;
        *dst += amt;
        Ok(())
    }
    pub fn total(&self) -> i64 {
        // lock all in order for a consistent snapshot
        let gs: Vec<_> = self.accounts.iter().map(|a| a.lock().unwrap()).collect();
        gs.iter().map(|g| **g).sum()
    }
}

fn main() {
    let bank = Arc::new(Bank::new(&[1000; 10]));
    let hs: Vec<_> = (0..8u64)
        .map(|t| {
            let bank = bank.clone();
            thread::spawn(move || {
                let mut s = t * 2654435761 + 1;
                for _ in 0..20_000 {
                    s ^= s << 13; s ^= s >> 7; s ^= s << 17;
                    let (a, b) = ((s % 10) as usize, ((s >> 8) % 10) as usize);
                    let _ = bank.transfer(a, b, ((s >> 16) % 50) as i64);
                }
            })
        })
        .collect();
    for h in hs { h.join().unwrap(); }
    assert_eq!(bank.total(), 10_000);
    assert_eq!(bank.transfer(1, 1, 5), Err(TransferError::SameAccount));
    println!("money conserved: {}", bank.total());
}
