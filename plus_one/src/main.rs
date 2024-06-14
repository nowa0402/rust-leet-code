fn main() {
    struct Solution;
    impl Solution {
        pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
            let mut digits = digits.clone();
            let mut result = vec![];
            let mut plus = 1;
            // numが9であれば桁上り処理を行う。
            while let Some(num) = digits.pop() {
                let tmp = num + plus;
                match tmp % 10 {
                    0 => {
                        result.push(0);
                    }
                    _ => {
                        plus = 0;
                        result.push(tmp);
                    }
                }
            }
            // plusが1の時は入力値の配列が全て9であるとき。配列の最後に桁上りの1を足す
            if plus == 1 {
                result.push(1);
            }
            result.reverse();
            result

            // ボツ案：一旦整数に直す方法。オーバーフローした
            // digits.reverse();
            // let mut num: i32 = digits
            //     .into_iter()
            //     .enumerate()
            //     .map(|x| {
            //         x.1 * 10_i32.pow(x.0.try_into().unwrap())
            //     })
            //     .sum();
            // num += 1;

            // let mut result: Vec<i32> = Vec::new();
            // while num > 9 {
            //     result.push(num % 10);
            //     num /= 10;
            // }
            // result.push(num);
            // result.reverse();
            // result
        }
    }
    println!("case1:{:?}", Solution::plus_one(vec![1, 2, 3]));
    println!("case2:{:?}", Solution::plus_one(vec![4, 3, 2, 2]));
    println!("case3:{:?}", Solution::plus_one(vec![9]));
    println!("case4:{:?}", Solution::plus_one(vec![9, 9]));
    println!("case4:{:?}", Solution::plus_one(vec![0]));
    println!(
        "case5:{:?}",
        Solution::plus_one(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    );
}
