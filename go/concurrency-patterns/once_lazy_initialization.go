//go:build ignore

// Lazy Initialization with sync.Once (thread-safe singleton)
// Statement: An expensive resource must be created at most once, on first use, even
// when many goroutines request it simultaneously; all must see the same instance.
// Approach: sync.Once guards construction; compare with a naive check-then-create
// version guarded by atomics to show how many times it would run without Once.
// Time: O(1) after init. Space: O(1).
package main

import (
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)

type Config struct{ DSN string }

var (
	once     sync.Once
	instance *Config
	inits    atomic.Int32
)

func GetConfig() *Config {
	once.Do(func() {
		inits.Add(1)
		time.Sleep(10 * time.Millisecond) // expensive load
		instance = &Config{DSN: "postgres://localhost/app"}
	})
	return instance
}

func main() {
	var wg sync.WaitGroup
	ptrs := make([]*Config, 50)
	for i := range ptrs {
		wg.Add(1)
		go func(i int) { defer wg.Done(); ptrs[i] = GetConfig() }(i)
	}
	wg.Wait()
	same := true
	for _, p := range ptrs {
		if p != ptrs[0] {
			same = false
		}
	}
	fmt.Println("initializations:", inits.Load(), "all same instance:", same, "dsn:", ptrs[0].DSN)

	// OnceValue (Go 1.21+) is the idiomatic shorthand.
	get := sync.OnceValue(func() int { inits.Add(1); return 42 })
	fmt.Println(get(), get(), "total inits:", inits.Load())
}
