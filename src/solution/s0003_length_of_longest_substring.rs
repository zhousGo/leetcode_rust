use std::collections::VecDeque;

/*
    给定一个字符串，请你找出其中不含有重复字符的最长子串的长度。



示例1:

输入: s = "abcabcbb"
输出: 3
解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
示例 2:

输入: s = "bbbbb"
输出: 1
解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
示例 3:

输入: s = "pwwkew"
输出: 3
解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
    请注意，你的答案必须是 子串 的长度，"pwke"是一个子序列，不是子串。
示例 4:

输入: s = ""
输出: 0


提示：

0 <= s.length <= 5 * 104
s由英文字母、数字、符号和空格组成

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/longest-substring-without-repeating-characters
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        let mut last: [i32; 128] = [-1; 128];
        let mut left = -1;
        let mut ans = 0;
        for (i, v) in s.chars().enumerate() {
            left = max(left, last[v as usize]);
            last[v as usize] = i as i32;
            ans = max(ans, (i as i32) - left);
        }
        return ans;
    }

    pub fn length_of_longest_substring_01(s: String) -> i32 {
        let mut dque = VecDeque::new();
        let mut ans = vec![];
        for c in s.chars() {
            for i in 0..dque.len() {
                if dque[i] == c {
                    ans.push(dque.len());
                    for _ in 0..i + 1 {
                        dque.pop_front();
                    }
                    break;
                }
            }
            dque.push_back(c);
        }
        ans.push(dque.len());
        let mut max = 0;
        for i in ans {
            if i > max {
                max = i;
            }
        }
        max as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn test() {
        assert_eq!(2, Solution::length_of_longest_substring("addc".to_string()));
        assert_eq!(
            2,
            Solution::length_of_longest_substring_01("addc".to_string())
        );
    }
}
