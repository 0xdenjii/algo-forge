# algo-forge

Standalone solutions to algorithm, data-structure, concurrency and design problems in Python, Rust, TypeScript and Go. Each file solves one problem, is runnable on its own, and is usually named `NNNN_problem_name.<ext>`.

## Layout

```
algo-forge/
  python/<topic>/      *.py
  rust/<topic>/        *.rs
  typescript/<topic>/  *.ts
  go/<topic>/          *.go
```

## Topics and file counts

Counted with `find <dir> -type f ! -name README.md | wc -l`.

| Language | Topic | Files |
|---|---|---|
| Python | arrays-and-hashing | 36 |
| Python | backtracking | 40 |
| Python | bit-manipulation-and-tries | 41 |
| Python | dynamic-programming | 39 |
| Python | graphs | 40 |
| Python | heaps-and-greedy | 35 |
| Python | strings | 37 |
| Python | trees-and-bst | 40 |
| Python | two-pointers-sliding-window | 39 |
| Rust | binary-search | 39 |
| Rust | competitive-programming | 40 |
| Rust | concurrency-problems | 30 |
| Rust | data-structures-from-scratch | 32 |
| Rust | dp-advanced | 34 |
| Rust | geometry | 33 |
| Rust | graphs-advanced | 30 |
| Rust | linked-lists-stacks-queues | 42 |
| Rust | number-theory-math | 35 |
| Rust | segment-trees-fenwick | 37 |
| Rust | string-algorithms | 35 |
| TypeScript | design-patterns-behavioral | 34 |
| TypeScript | design-patterns-creational | 35 |
| TypeScript | design-patterns-structural | 33 |
| TypeScript | object-oriented-design | 38 |
| TypeScript | system-design-lld | 32 |
| Go | concurrency-patterns | 40 |
| | **Total** | **946** |

By language: Python 347, Rust 387, TypeScript 172, Go 40.

## Running code

**Python** (3.10+):
```sh
python python/strings/<file>.py
```

**Rust** (each file has its own `main`; no Cargo project needed):
```sh
rustc -O rust/geometry/<file>.rs -o target/sol && ./target/sol
```

**TypeScript** (Node 18+):
```sh
npx tsx typescript/system-design-lld/<file>.ts
```

**Go** (1.21+; files carry `//go:build ignore`, so run them individually):
```sh
go run go/concurrency-patterns/<file>.go
```
