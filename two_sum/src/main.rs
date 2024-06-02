/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
/// You can return the answer in any order.
// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]
// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]

// Constraints:

// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.
// use core::num;
use std::collections::HashMap;

fn main() {
    #[derive(Debug)]
    struct Solution;
    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            // よくあるforで追うアルゴリズム
            // for (i, i_num) in nums.iter().enumerate() {
            //     println!("{}, {}", i, i_num);
            //     for (j, j_num) in nums.iter().enumerate() {
            //         println!("{}, {}", j, j_num);
            //         if j <= i {
            //             continue;
            //         }
            //         if i_num + j_num == target {
            //             return [i as i32, j as i32].to_vec();
            //         }
            //     }
            // }

            // ハッシュマップでやるとこうなる
            let mut hash_map = HashMap::new();
            for (index, num) in nums.iter().enumerate() {
                let n = target - num;
                if hash_map.get(&n).is_none() {
                    hash_map.insert(num, index as i32);
                } else {
                    return [*hash_map.get(&n).unwrap(), index as i32].to_vec();
                }
            }

            nums
        }
    }
    let result = Solution::two_sum([3, 2, 4].to_vec(), 6);
    println!("{:?}", result);
}
