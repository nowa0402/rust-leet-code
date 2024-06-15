fn main() {
    struct Solution;
    impl Solution {
        pub fn single_number(nums: Vec<i32>) -> i32 {
            let mut result = 0;
            // 排他的論理和（XOR）で解く
            for num in nums {
                result ^= num;
            }
            result
        }
    }
    let case1 = vec![2, 2, 1];
    println!("case1:{}", Solution::single_number(case1));
    let case2 = vec![4, 1, 2, 1, 2];
    println!("case2:{}", Solution::single_number(case2));
    let case3 = vec![1];
    println!("case3:{}", Solution::single_number(case3));
}
