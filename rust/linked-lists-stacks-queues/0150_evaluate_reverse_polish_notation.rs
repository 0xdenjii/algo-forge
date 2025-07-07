// LeetCode 150 - Evaluate Reverse Polish Notation
// Evaluate an RPN expression with + - * / (integer division truncates toward zero).
// Approach: operand stack; on an operator pop two, apply, push the result.
// Errors (bad token, underflow, division by zero) surface as Err instead of panics.
// Time O(n), Space O(n).
pub fn eval_rpn(tokens: &[&str]) -> Result<i64, String> {
    let mut st: Vec<i64> = Vec::new();
    for &t in tokens {
        match t {
            "+" | "-" | "*" | "/" => {
                let b = st.pop().ok_or("stack underflow")?;
                let a = st.pop().ok_or("stack underflow")?;
                st.push(match t {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    _ => a.checked_div(b).ok_or("division by zero")?,
                });
            }
            num => st.push(num.parse().map_err(|_| format!("bad token {num:?}"))?),
        }
    }
    match st.as_slice() { [v] => Ok(*v), _ => Err("malformed expression".into()) }
}

fn main() {
    assert_eq!(eval_rpn(&["2", "1", "+", "3", "*"]), Ok(9));
    assert_eq!(eval_rpn(&["4", "13", "5", "/", "+"]), Ok(6));
    assert_eq!(eval_rpn(&["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]), Ok(22));
    assert!(eval_rpn(&["1", "+"]).is_err());
    println!("ok");
}
