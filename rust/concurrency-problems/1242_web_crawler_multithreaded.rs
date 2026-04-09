// LeetCode 1242 - Web Crawler Multithreaded
// Starting from startUrl, crawl all URLs reachable via HtmlParser.getUrls that share
// startUrl's hostname. Return them in any order; crawling must be concurrent.
// Approach: a shared visited set plus a work queue; N worker threads pop URLs, fetch
// links, and push unseen same-host links. An in-flight counter detects termination.
// Time: O(V + E) total work. Space: O(V).
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

trait HtmlParser: Send + Sync {
    fn get_urls(&self, url: &str) -> Vec<String>;
}

struct FakeParser(HashMap<String, Vec<String>>);
impl HtmlParser for FakeParser {
    fn get_urls(&self, url: &str) -> Vec<String> {
        self.0.get(url).cloned().unwrap_or_default()
    }
}

fn host(url: &str) -> &str {
    let rest = url.strip_prefix("http://").unwrap_or(url);
    rest.split('/').next().unwrap_or("")
}

struct Shared {
    queue: VecDeque<String>,
    seen: HashSet<String>,
    in_flight: usize,
}

fn crawl(start: &str, parser: Arc<dyn HtmlParser>, workers: usize) -> Vec<String> {
    let target = host(start).to_string();
    let state = Arc::new((
        Mutex::new(Shared {
            queue: VecDeque::from([start.to_string()]),
            seen: HashSet::from([start.to_string()]),
            in_flight: 0,
        }),
        Condvar::new(),
    ));
    let hs: Vec<_> = (0..workers)
        .map(|_| {
            let (state, parser, target) = (state.clone(), parser.clone(), target.clone());
            thread::spawn(move || loop {
                let (lock, cv) = &*state;
                let url = {
                    let mut s = cv.wait_while(lock.lock().unwrap(), |s| s.queue.is_empty() && s.in_flight > 0).unwrap();
                    match s.queue.pop_front() {
                        Some(u) => { s.in_flight += 1; u }
                        None => return, // empty and nothing in flight: done
                    }
                };
                let links = parser.get_urls(&url); // done outside the lock
                let mut s = lock.lock().unwrap();
                for l in links {
                    if host(&l) == target && s.seen.insert(l.clone()) {
                        s.queue.push_back(l);
                    }
                }
                s.in_flight -= 1;
                cv.notify_all();
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    let mut v: Vec<_> = state.0.lock().unwrap().seen.iter().cloned().collect();
    v.sort();
    v
}

fn main() {
    let g: HashMap<String, Vec<String>> = [
        ("http://news.yahoo.com/news/topics/", vec!["http://news.yahoo.com", "http://news.yahoo.com/news"]),
        ("http://news.yahoo.com", vec!["http://news.yahoo.com/us", "http://news.google.com"]),
        ("http://news.yahoo.com/news", vec!["http://news.yahoo.com/news/topics/"]),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v.into_iter().map(String::from).collect()))
    .collect();
    let res = crawl("http://news.yahoo.com/news/topics/", Arc::new(FakeParser(g)), 4);
    assert_eq!(res.len(), 4);
    println!("{res:#?}");
}
