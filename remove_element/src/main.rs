fn main() {
    struct Solution;
    impl Solution {
        pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
            let mut tmp = Vec::new();
            while let Some(i) = nums.pop() {
                if i != val {
                    tmp.push(i);
                }
            }
            while let Some(i) = tmp.pop() {
                nums.push(i);
            }
            nums.len() as i32
        }
    }
    let mut case1 = vec![3, 2, 2, 3];
    println!("case1:{}", Solution::remove_element(&mut case1, 3));
    let mut case2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
    println!("case2:{}", Solution::remove_element(&mut case2, 2));
}
