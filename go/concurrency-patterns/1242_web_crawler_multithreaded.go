//go:build ignore

// LeetCode 1242 - Web Crawler Multithreaded
// Statement: From startUrl, crawl every URL reachable through HtmlParser.GetUrls that
// shares startUrl's hostname, concurrently, visiting each URL exactly once.
// Approach: concurrent graph traversal; a mutex-guarded visited set plus a WaitGroup
// counting in-flight fetches; each new same-host URL gets its own goroutine.
// Time: O(V+E) total work. Space: O(V).
package main

import (
	"fmt"
	"sort"
	"strings"
	"sync"
	"time"
)

type HtmlParser map[string][]string

func (h HtmlParser) GetUrls(u string) []string { time.Sleep(5 * time.Millisecond); return h[u] }

func host(u string) string { return strings.SplitN(strings.TrimPrefix(u, "http://"), "/", 2)[0] }

func crawl(start string, p HtmlParser) []string {
	h := host(start)
	var mu sync.Mutex
	seen := map[string]bool{start: true}
	var wg sync.WaitGroup
	var visit func(string)
	visit = func(u string) {
		defer wg.Done()
		for _, nx := range p.GetUrls(u) {
			if host(nx) != h {
				continue
			}
			mu.Lock()
			if !seen[nx] {
				seen[nx] = true
				wg.Add(1)
				go visit(nx)
			}
			mu.Unlock()
		}
	}
	wg.Add(1)
	go visit(start)
	wg.Wait()
	out := make([]string, 0, len(seen))
	for u := range seen {
		out = append(out, u)
	}
	sort.Strings(out)
	return out
}

func main() {
	p := HtmlParser{
		"http://news.yahoo.com/news/topics/": {"http://news.yahoo.com", "http://news.yahoo.com/news"},
		"http://news.yahoo.com":              {"http://news.yahoo.com/us", "http://news.google.com"},
		"http://news.yahoo.com/news":         {"http://news.yahoo.com/news/topics/"},
		"http://news.google.com":             {"http://news.yahoo.com"},
	}
	for _, u := range crawl("http://news.yahoo.com/news/topics/", p) {
		fmt.Println(u)
	}
}
