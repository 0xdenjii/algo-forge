// LeetCode 71 - Simplify Path
// Convert an absolute Unix path into its canonical form (resolve ".", "..", repeated "/").
// Approach: split on '/', use a stack of directory names; ".." pops, "." and "" are skipped.
// Time O(n), Space O(n).
pub fn simplify_path(path: &str) -> String {
    let mut st: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => { st.pop(); }
            name => st.push(name),
        }
    }
    format!("/{}", st.join("/"))
}

fn main() {
    assert_eq!(simplify_path("/home/"), "/home");
    assert_eq!(simplify_path("/../"), "/");
    assert_eq!(simplify_path("/a/./b/../../c/"), "/c");
    assert_eq!(simplify_path("/home//foo/.../"), "/home/foo/...");
    println!("ok");
}
