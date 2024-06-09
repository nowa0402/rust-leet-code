fn main() {
    struct Solution;
    impl Solution {
        pub fn is_valid(s: String) -> bool {
            let mut stack: Vec<char> = Vec::new();
            for c in s.chars() {
                match c {
                    ')' => {
                        if stack.is_empty() {
                            return false;
                        }
                        if stack.pop().unwrap() != '(' {
                            return false;
                        }
                    }
                    ']' => {
                        if stack.is_empty() {
                            return false;
                        }
                        if stack.pop().unwrap() != '[' {
                            return false;
                        }
                    }
                    '}' => {
                        if stack.is_empty() {
                            return false;
                        }
                        if stack.pop().unwrap() != '{' {
                            return false;
                        }
                    }
                    _ => stack.push(c),
                }
            }
            stack.is_empty()
        }
    }

    println!("case1:{}", Solution::is_valid(String::from("()")));
    println!("case1:{}", Solution::is_valid(String::from("()[]{}")));
    println!("case1:{}", Solution::is_valid(String::from("(}")));
    println!("case4:{}", Solution::is_valid(String::from("([{}])")));
}
