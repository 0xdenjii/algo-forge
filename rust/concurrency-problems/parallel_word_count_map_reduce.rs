// Parallel algorithm - Word Count (MapReduce)
// Given many lines of text, count word frequencies using all cores.
// Approach: map phase - split the input into chunks, each scoped thread builds a
// local HashMap (no shared locking); reduce phase - merge the partial maps.
// Time: O(total words). Space: O(distinct words * threads).
use std::collections::HashMap;
use std::thread;

pub fn word_count(lines: &[&str], workers: usize) -> HashMap<String, usize> {
    let chunk = lines.len().div_ceil(workers.max(1)).max(1);
    let partials: Vec<HashMap<String, usize>> = thread::scope(|s| {
        let hs: Vec<_> = lines
            .chunks(chunk)
            .map(|part| {
                s.spawn(move || {
                    let mut m = HashMap::new();
                    for line in part {
                        for w in line.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()) {
                            *m.entry(w.to_lowercase()).or_insert(0) += 1;
                        }
                    }
                    m
                })
            })
            .collect();
        hs.into_iter().map(|h| h.join().unwrap()).collect()
    });
    partials.into_iter().fold(HashMap::new(), |mut acc, m| {
        for (k, v) in m {
            *acc.entry(k).or_insert(0) += v;
        }
        acc
    })
}

fn main() {
    let text = "the quick brown fox\njumps over the lazy dog\nThe dog barks\nfox and dog";
    let lines: Vec<&str> = text.lines().collect();
    let counts = word_count(&lines, 3);
    assert_eq!(counts["the"], 3);
    assert_eq!(counts["dog"], 3);
    let mut top: Vec<_> = counts.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    println!("{:?}", &top[..4]);
}
