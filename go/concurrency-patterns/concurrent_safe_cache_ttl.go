//go:build ignore

// Concurrent-Safe TTL Cache with Janitor
// Statement: A key/value cache safe for concurrent use where entries expire after a
// TTL; expired entries are invisible on Get and periodically purged in background.
// Approach: RWMutex-guarded map with expiry timestamps; a janitor goroutine driven by
// a ticker deletes expired keys and stops via Close().
// Time: O(1) Get/Set, O(n) per sweep. Space: O(n).
package main

import (
	"fmt"
	"sync"
	"time"
)

type entry[V any] struct {
	val V
	exp time.Time
}

type Cache[K comparable, V any] struct {
	mu   sync.RWMutex
	data map[K]entry[V]
	ttl  time.Duration
	stop chan struct{}
	once sync.Once
}

func NewCache[K comparable, V any](ttl, sweep time.Duration) *Cache[K, V] {
	c := &Cache[K, V]{data: map[K]entry[V]{}, ttl: ttl, stop: make(chan struct{})}
	go func() {
		t := time.NewTicker(sweep)
		defer t.Stop()
		for {
			select {
			case now := <-t.C:
				c.mu.Lock()
				for k, e := range c.data {
					if now.After(e.exp) {
						delete(c.data, k)
					}
				}
				c.mu.Unlock()
			case <-c.stop:
				return
			}
		}
	}()
	return c
}

func (c *Cache[K, V]) Set(k K, v V) {
	c.mu.Lock()
	c.data[k] = entry[V]{v, time.Now().Add(c.ttl)}
	c.mu.Unlock()
}

func (c *Cache[K, V]) Get(k K) (V, bool) {
	c.mu.RLock()
	e, ok := c.data[k]
	c.mu.RUnlock()
	if !ok || time.Now().After(e.exp) {
		var zero V
		return zero, false
	}
	return e.val, true
}

func (c *Cache[K, V]) Len() int { c.mu.RLock(); defer c.mu.RUnlock(); return len(c.data) }

func (c *Cache[K, V]) Close() { c.once.Do(func() { close(c.stop) }) }

func main() {
	c := NewCache[string, int](40*time.Millisecond, 10*time.Millisecond)
	defer c.Close()
	var wg sync.WaitGroup
	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func(i int) { defer wg.Done(); c.Set(fmt.Sprint("k", i%10), i) }(i)
	}
	wg.Wait()
	_, ok := c.Get("k3")
	fmt.Println("len after writes:", c.Len(), "k3 present:", ok)
	time.Sleep(80 * time.Millisecond)
	_, ok = c.Get("k3")
	fmt.Println("len after ttl:", c.Len(), "k3 present:", ok)
}
