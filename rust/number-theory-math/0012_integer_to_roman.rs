// LeetCode 12 - Integer to Roman
// Convert an integer in [1, 3999] to a Roman numeral.
// Approach: greedy over the 13 value/symbol pairs, including subtractive forms (CM, XC, IV...).
// Time: O(1) (bounded output), Space: O(1)
fn int_to_roman(mut n: u32) -> String {
    const TABLE: [(u32, &str); 13] = [
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
    ];
    let mut s = String::new();
    for &(v, sym) in &TABLE {
        while n >= v {
            s.push_str(sym);
            n -= v;
        }
    }
    s
}

fn main() {
    assert_eq!(int_to_roman(58), "LVIII");
    assert_eq!(int_to_roman(1994), "MCMXCIV");
    assert_eq!(int_to_roman(3749), "MMMDCCXLIX");
    println!("ok");
}
