/*
    给定一个整数数组 num和一个目标值 target，请你在该数组中找出和为目标值的那两个整数，并返回他们的数组下标。
你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。

示例:

给定 nums = [2, 7, 11, 15], target = 9

因为 nums[0] + nums[1] = 2 + 7 = 9
所以返回 [0, 1]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/two-sum
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        // 遍历数组
        for (index, &num) in nums.iter().enumerate() {
            // 如果当前map中有值为(target - num)的数，即找到符合条件的两个数，返回他们的下标
            if let Some(&pos) = map.get(&(target - num)) {
                return vec![pos as i32, index as i32];
            }
            // 如果没有，则将当前数的值和下标存入map，用于下次计算比较
            map.insert(num, index);
        }
        vec![]
    }
}

#[cfg(test)]
mod test {
    use crate::solution::s0001_two_sum::Solution;

    #[test]
    fn test() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
