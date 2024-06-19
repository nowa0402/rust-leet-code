fn main() {
    struct Solution;
    impl Solution {
        pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            // 自前実装
            // let mut tmp: Vec<usize> = Vec::new();
            // let mut prev: Option<i32> = None;
            // for num in nums.iter().enumerate() {
            //     match prev {
            //         Some(i) => {
            //             if i == *num.1 {
            //                 tmp.push(num.0);
            //             } else {
            //                 prev = Some(*num.1);
            //             }
            //         }
            //         None => prev = Some(*num.1),
            //     }
            // }
            // while let Some(i) = tmp.pop() {
            //     nums.remove(i);
            // }
            // println!("nums:{:?}, len:{}", nums, nums.len());

            // 用意されているメソッドを使用する
            nums.dedup();
            nums.len() as i32
        }
    }
    let mut case1 = vec![1, 1, 2];
    println!("case1:{}", Solution::remove_duplicates(&mut case1));
    let mut case2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    println!("case2:{}", Solution::remove_duplicates(&mut case2));
}
