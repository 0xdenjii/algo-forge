//go:build ignore

// Parallel Merge Sort
// Statement: Sort a slice of integers using multiple goroutines for the recursive
// halves, without spawning unbounded goroutines for tiny subarrays.
// Approach: recurse in parallel while depth > 0 (depth ~ log2(GOMAXPROCS)), then fall
// back to sequential merge sort; merge into a shared scratch buffer.
// Time: O(n log n) work, ~O(n) span with enough cores. Space: O(n).
package main

import (
	"fmt"
	"math/rand"
	"runtime"
	"sort"
	"sync"
)

func merge(a, tmp []int, mid int) {
	copy(tmp, a)
	i, j, k := 0, mid, 0
	for i < mid && j < len(a) {
		if tmp[i] <= tmp[j] {
			a[k] = tmp[i]
			i++
		} else {
			a[k] = tmp[j]
			j++
		}
		k++
	}
	for i < mid {
		a[k] = tmp[i]
		i++
		k++
	}
	for j < len(a) {
		a[k] = tmp[j]
		j++
		k++
	}
}

func msort(a, tmp []int, depth int) {
	if len(a) < 2 {
		return
	}
	mid := len(a) / 2
	if depth > 0 && len(a) > 2048 {
		var wg sync.WaitGroup
		wg.Add(1)
		go func() { defer wg.Done(); msort(a[:mid], tmp[:mid], depth-1) }()
		msort(a[mid:], tmp[mid:], depth-1)
		wg.Wait()
	} else {
		msort(a[:mid], tmp[:mid], 0)
		msort(a[mid:], tmp[mid:], 0)
	}
	merge(a, tmp, mid)
}

func ParallelSort(a []int) {
	depth := 0
	for p := runtime.GOMAXPROCS(0); p > 1; p >>= 1 {
		depth++
	}
	msort(a, make([]int, len(a)), depth+1)
}

func main() {
	r := rand.New(rand.NewSource(1))
	a := make([]int, 200000)
	for i := range a {
		a[i] = r.Intn(1000000)
	}
	ParallelSort(a)
	fmt.Println("sorted:", sort.IntsAreSorted(a), "first:", a[:5])
}
