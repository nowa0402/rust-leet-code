fn main() {
    struct Solution;
    impl Solution {
        pub fn roman_to_int(s: String) -> i32 {
            let mut result = 0;
            let mut prev = 0;
            for c in s.chars() {
                let current = match c {
                    'I' => 1,
                    'V' => 5,
                    'X' => 10,
                    'L' => 50,
                    'C' => 100,
                    'D' => 500,
                    'M' => 1000,
                    _ => 0,
                };

                if prev != 0 && prev < current {
                    result += current - prev * 2;
                } else {
                    result += current;
                }
                prev = current;
            }
            result
        }
    }
    println!("case1: {}", Solution::roman_to_int(String::from("III")));
    println!("case1: {}", Solution::roman_to_int(String::from("LVIII")));
    println!("case1: {}", Solution::roman_to_int(String::from("MCMXCIV")));
}
