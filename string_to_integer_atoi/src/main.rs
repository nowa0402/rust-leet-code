fn main() {
    struct Solution;
    impl Solution {
        pub fn my_atoi(s: String) -> i32 {
            let mut tmp = String::new();
            // 文字を足していく
            // 数値が来る前かつtmpが空の場合のみ空文字, +, -だけ許容する
            for c in s.chars() {
                if (c == '-' || c == '+') && tmp.is_empty() {
                    tmp.push_str(&c.to_string());
                    continue;
                }
                if c == ' ' && tmp.is_empty() {
                    continue;
                }
                if !c.is_ascii_digit() {
                    break;
                }
                tmp.push_str(&c.to_string())
            }
            // 空文字もしくは記号だけなら0を返却し、それ以外は数字に変換
            if tmp.is_empty() {
                return 0;
            }
            if tmp == "+" || tmp == "-" {
                return 0;
            }
            if tmp.starts_with('-') {
                let num: i32 = tmp.parse().unwrap_or(std::i32::MIN);
                num
            } else {
                let num: i32 = tmp.parse().unwrap_or(std::i32::MAX);
                num
            }
        }
    }
    println!("case1:{}", Solution::my_atoi("42".to_string()));
    println!("case2:{}", Solution::my_atoi(" -042".to_string()));
    println!("case3:{}", Solution::my_atoi("1337c0d3".to_string()));
    println!("case4:{}", Solution::my_atoi("0-1".to_string()));
    println!("case5:{}", Solution::my_atoi("words and 987".to_string()));
    println!("case6:{}", Solution::my_atoi("+-12".to_string()));
}
