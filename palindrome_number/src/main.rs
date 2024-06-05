fn main() {
    pub struct Solution;
    impl Solution {
        pub fn is_palindrome(x: i32) -> bool {
            if x < 0 {
                return false;
            }
            if x < 10 {
                return true;
            }

            // 数字を配列に分解
            let mut split_num: Vec<i32> = Vec::new();
            let mut n = x;
            while n > 9 {
                split_num.push(n % 10);
                n /= 10;
            }
            split_num.push(n);

            // 最初と最後の要素を中央地点にくるまで比較し続ける
            for count in 0..split_num.len() as i32 / 2 {
                let first = split_num[count as usize];
                let last = split_num.pop().unwrap();
                if first != last {
                    return false;
                }
            }
            true
        }
    }
    println!("case1: {}", Solution::is_palindrome(121));
    println!("case2: {}", Solution::is_palindrome(-121));
    println!("case3: {}", Solution::is_palindrome(10));
    println!("case4: {}", Solution::is_palindrome(12122));
    println!("case5: {}", Solution::is_palindrome(223322));
}
