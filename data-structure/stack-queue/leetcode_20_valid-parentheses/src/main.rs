struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() == 0 {
            return true;
        }

        let mut stack: Vec<char> = Vec::new();
        for i in 0..chars.len() {
            if chars[i] == '(' {
                stack.push(')');
            } else if chars[i] == '[' {
                stack.push(']');
            } else if chars[i] == '{' {
                stack.push('}');
            } else if stack.is_empty() || chars[i] != stack.pop().unwrap() {
                return false;
            }
        }

        return stack.is_empty();
    }
}

fn main() {
    let parenthes = String::from("{[]}");
    println!("{}", Solution::is_valid(parenthes));
}
