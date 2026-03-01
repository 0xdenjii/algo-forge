// Tarjan's Strongly Connected Components (classic)
// Partition a directed graph into maximal strongly connected components.
// Approach: iterative Tarjan (explicit call stack avoids recursion overflow) tracking
// discovery index and low-link; pop a component when low[v] == idx[v].
// Components are emitted in reverse topological order of the condensation.
// Time: O(V + E), Space: O(V)
pub fn tarjan_scc(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    const UNSEEN: usize = usize::MAX;
    let n = adj.len();
    let (mut idx, mut low) = (vec![UNSEEN; n], vec![0; n]);
    let mut on_stack = vec![false; n];
    let (mut stack, mut comps, mut counter) = (Vec::new(), Vec::new(), 0);
    for s in 0..n {
        if idx[s] != UNSEEN {
            continue;
        }
        idx[s] = counter;
        low[s] = counter;
        counter += 1;
        stack.push(s);
        on_stack[s] = true;
        let mut call: Vec<(usize, usize)> = vec![(s, 0)];
        while let Some(&(v, i)) = call.last() {
            if i < adj[v].len() {
                call.last_mut().unwrap().1 += 1;
                let w = adj[v][i];
                if idx[w] == UNSEEN {
                    idx[w] = counter;
                    low[w] = counter;
                    counter += 1;
                    stack.push(w);
                    on_stack[w] = true;
                    call.push((w, 0));
                } else if on_stack[w] {
                    low[v] = low[v].min(idx[w]);
                }
            } else {
                call.pop();
                if let Some(&(p, _)) = call.last() {
                    low[p] = low[p].min(low[v]);
                }
                if low[v] == idx[v] {
                    let mut comp = Vec::new();
                    loop {
                        let w = stack.pop().unwrap();
                        on_stack[w] = false;
                        comp.push(w);
                        if w == v {
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
    let adj = vec![vec![1], vec![2], vec![0, 3], vec![4], vec![5], vec![3], vec![]];
    let mut c: Vec<Vec<usize>> = tarjan_scc(&adj)
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
