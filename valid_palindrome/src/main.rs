fn main() {
    struct Solution;
    impl Solution {
        pub fn is_palindrome(s: String) -> bool {
            let mut chars: Vec<char> = s
                .to_ascii_lowercase()
                .as_str()
                .chars()
                .filter(|x| matches!(x, 'a'..='z' | '0'..='9'))
                .collect();
            if chars.is_empty() || chars.len() == 1 {
                return true;
            }
            for count in 0..chars.len() as i32 / 2 {
                let first = chars[count as usize];
                let last = chars.pop().unwrap();
                if first != last {
                    return false;
                }
            }
            true
        }
    }
    let case1 = "A man, a plan, a canal: Panama";
    println!("case1:{}", Solution::is_palindrome(case1.to_string()));
    let case2 = "race a car";
    println!("case2:{}", Solution::is_palindrome(case2.to_string()));
    let case3 = " ";
    println!("case3:{}", Solution::is_palindrome(case3.to_string()));
}
