// Title: Strongly Connected Components (Tarjan)
// Problem: Given a directed graph, partition its vertices into strongly connected components.
// Approach: Tarjan's single-pass DFS with discovery index and lowlink, written iteratively
// (explicit call stack) so deep graphs cannot overflow the native stack.
// Complexity: O(V + E) time and space.
pub fn tarjan_scc(g: &[Vec<usize>]) -> Vec<Vec<usize>> {
    const UNSEEN: usize = usize::MAX;
    let n = g.len();
    let mut idx = vec![UNSEEN; n];
    let mut low = vec![0; n];
    let mut on_stack = vec![false; n];
    let mut stack = Vec::new();
    let mut comps = Vec::new();
    let mut timer = 0;
    for s in 0..n {
        if idx[s] != UNSEEN {
            continue;
        }
        let mut call: Vec<(usize, usize)> = vec![(s, 0)];
        idx[s] = timer;
        low[s] = timer;
        timer += 1;
        stack.push(s);
        on_stack[s] = true;
        while let Some(frame) = call.last_mut() {
            let (u, ei) = *frame;
            if ei < g[u].len() {
                frame.1 += 1;
                let v = g[u][ei];
                if idx[v] == UNSEEN {
                    idx[v] = timer;
                    low[v] = timer;
                    timer += 1;
                    stack.push(v);
                    on_stack[v] = true;
                    call.push((v, 0));
                } else if on_stack[v] {
                    low[u] = low[u].min(idx[v]);
                }
            } else {
                call.pop();
                if let Some(&(p, _)) = call.last() {
                    low[p] = low[p].min(low[u]);
                }
                if low[u] == idx[u] {
                    let mut comp = Vec::new();
                    loop {
                        let w = stack.pop().unwrap();
                        on_stack[w] = false;
                        comp.push(w);
                        if w == u {
                            break;
                        }
                    }
                    comps.push(comp);
                }
            }
        }
    }
    comps
}

fn main() {
    let g = vec![vec![1], vec![2], vec![0, 3], vec![4], vec![5], vec![3], vec![]];
    let mut c: Vec<Vec<usize>> = tarjan_scc(&g)
        .into_iter()
        .map(|mut v| {
            v.sort();
            v
        })
        .collect();
    c.sort();
    assert_eq!(c, vec![vec![0, 1, 2], vec![3, 4, 5], vec![6]]);
    println!("ok");
}
