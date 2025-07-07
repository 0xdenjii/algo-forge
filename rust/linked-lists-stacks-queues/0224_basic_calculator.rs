// LeetCode 224 - Basic Calculator
// Evaluate an expression with integers, '+', '-', unary minus, parentheses and spaces.
// Approach: running result and sign; on '(' push (result, sign) and reset;
// on ')' pop and fold the inner result back in.
// Time O(n), Space O(depth).
pub fn calculate(s: &str) -> i64 {
    let mut st: Vec<(i64, i64)> = Vec::new();
    let (mut result, mut sign, mut num) = (0i64, 1i64, 0i64);
    for c in s.chars() {
        match c {
            '0'..='9' => num = num * 10 + c.to_digit(10).unwrap() as i64,
            '+' | '-' => {
                result += sign * num;
                num = 0;
                sign = if c == '+' { 1 } else { -1 };
            }
            '(' => { st.push((result, sign)); result = 0; sign = 1; }
            ')' => {
                result += sign * num;
                num = 0;
                let (prev, prev_sign) = st.pop().expect("unbalanced parentheses");
                result = prev + prev_sign * result;
            }
            _ => {}
        }
    }
    result + sign * num
}

fn main() {
    assert_eq!(calculate("1 + 1"), 2);
    assert_eq!(calculate(" 2-1 + 2 "), 3);
    assert_eq!(calculate("(1+(4+5+2)-3)+(6+8)"), 23);
    assert_eq!(calculate("-(2 + 3)"), -5);
    println!("ok");
}
