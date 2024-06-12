fn main() {
    struct Solution;
    impl Solution {
        pub fn str_str(haystack: String, needle: String) -> i32 {
            // 標準ライブラリを使ったやり方
            match haystack.find(&needle) {
                Some(i) => i as i32,
                None => -1,
            }
            // TODO: アルゴリズムでやる方法も後で実装する
        }
    }
    println!(
        "case1: {}",
        Solution::str_str("sadbutsad".to_string(), "sad".to_string())
    );
    println!(
        "case2: {}",
        Solution::str_str("leetcode".to_string(), "leeto".to_string())
    );
}
