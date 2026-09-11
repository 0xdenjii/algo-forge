//go:build ignore

// Deadlock-Free Bank Transfers
// Statement: Concurrent transfers between accounts each lock two accounts. Naive
// locking (from, then to) deadlocks when A->B and B->A run together. Make it safe
// and keep total money conserved.
// Approach: lock ordering by account ID (always lower ID first); reject overdrafts.
// Time: O(1) per transfer. Space: O(accounts).
package main

import (
	"fmt"
	"math/rand"
	"sync"
)

type Account struct {
	ID      int
	mu      sync.Mutex
	balance int
}

func Transfer(from, to *Account, amt int) bool {
	if from == to {
		return false
	}
	first, second := from, to
	if first.ID > second.ID {
		first, second = second, first
	}
	first.mu.Lock()
	defer first.mu.Unlock()
	second.mu.Lock()
	defer second.mu.Unlock()
	if from.balance < amt {
		return false
	}
	from.balance -= amt
	to.balance += amt
	return true
}

func main() {
	accts := make([]*Account, 5)
	for i := range accts {
		accts[i] = &Account{ID: i, balance: 1000}
	}
	var wg sync.WaitGroup
	for g := 0; g < 8; g++ {
		wg.Add(1)
		go func(seed int64) {
			defer wg.Done()
			r := rand.New(rand.NewSource(seed))
			for i := 0; i < 10000; i++ {
				Transfer(accts[r.Intn(5)], accts[r.Intn(5)], r.Intn(100))
			}
		}(int64(g))
	}
	wg.Wait()
	total := 0
	for _, a := range accts {
		total += a.balance
	}
	fmt.Println("total:", total, "conserved:", total == 5000)
}
