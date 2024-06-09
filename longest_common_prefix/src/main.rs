fn main() {
    struct Solution;
    impl Solution {
        pub fn longest_common_prefix(strs: Vec<String>) -> String {
            let first = strs.first().unwrap();
            let mut result = String::new();
            let mut tmp = String::new();

            for c in first.chars() {
                tmp.push_str(&c.to_string());
                for s in strs.iter() {
                    if !s.starts_with(&tmp) {
                        return result;
                    }
                }
                result.push_str(&c.to_string());
            }
            result
        }
    }
    let case1 = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    println!("case1:{}", Solution::longest_common_prefix(case1));
    let case2 = vec![
        String::from("dog"),
        String::from("racecar"),
        String::from("car"),
    ];
    println!("case2:{}", Solution::longest_common_prefix(case2));
}
