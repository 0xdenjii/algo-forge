//go:build ignore

// LeetCode 1226 - The Dining Philosophers
// Statement: Five philosophers share five forks; each needs both neighbouring forks
// to eat. Implement WantsToEat so there is no deadlock and every request finishes.
// Approach: resource ordering - always lock the lower-numbered fork first, which
// breaks the circular-wait condition.
// Time: O(1) per meal. Space: O(n) mutexes.
package main

import (
	"fmt"
	"sync"
)

type DiningPhilosophers struct{ forks [5]sync.Mutex }

func (d *DiningPhilosophers) WantsToEat(p int, pickL, pickR, eat, putL, putR func()) {
	a, b := p, (p+1)%5
	if a > b {
		a, b = b, a
	}
	d.forks[a].Lock()
	d.forks[b].Lock()
	pickL()
	pickR()
	eat()
	putL()
	putR()
	d.forks[b].Unlock()
	d.forks[a].Unlock()
}

func main() {
	var d DiningPhilosophers
	var mu sync.Mutex
	meals := make([]int, 5)
	var wg sync.WaitGroup
	noop := func() {}
	for p := 0; p < 5; p++ {
		for k := 0; k < 100; k++ {
			wg.Add(1)
			go func(p int) {
				defer wg.Done()
				d.WantsToEat(p, noop, noop, func() { mu.Lock(); meals[p]++; mu.Unlock() }, noop, noop)
			}(p)
		}
	}
	wg.Wait()
	fmt.Println("meals per philosopher:", meals)
}
