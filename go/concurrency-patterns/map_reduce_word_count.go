//go:build ignore

// MapReduce Word Count
// Statement: Count word frequencies across many documents using concurrent mappers
// and a reducer, producing the top words by count.
// Approach: each mapper builds a local map for one document (no sharing) and sends
// it on a channel; a single reducer merges maps; results sorted by count then word.
// Time: O(total words + U log U). Space: O(U) unique words.
package main

import (
	"fmt"
	"sort"
	"strings"
	"sync"
)

func wordCount(docs []string) map[string]int {
	partials := make(chan map[string]int)
	var wg sync.WaitGroup
	for _, d := range docs {
		wg.Add(1)
		go func(d string) {
			defer wg.Done()
			m := map[string]int{}
			for _, w := range strings.FieldsFunc(strings.ToLower(d), func(r rune) bool {
				return !(r >= 'a' && r <= 'z')
			}) {
				m[w]++
			}
			partials <- m
		}(d)
	}
	go func() { wg.Wait(); close(partials) }()
	total := map[string]int{}
	for m := range partials {
		for w, c := range m {
			total[w] += c
		}
	}
	return total
}

func main() {
	docs := []string{
		"The quick brown fox jumps over the lazy dog.",
		"The dog barks; the fox runs.",
		"Quick, quick! said the fox.",
	}
	counts := wordCount(docs)
	type kv struct {
		w string
		c int
	}
	var list []kv
	for w, c := range counts {
		list = append(list, kv{w, c})
	}
	sort.Slice(list, func(i, j int) bool {
		if list[i].c != list[j].c {
			return list[i].c > list[j].c
		}
		return list[i].w < list[j].w
	})
	for _, e := range list[:5] {
		fmt.Printf("%-6s %d\n", e.w, e.c)
	}
}
