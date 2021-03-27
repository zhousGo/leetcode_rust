/**
 * 给你一个字符串 s，找到 s 中最长的回文子串。

示例 1：

输入：s = "babad"
输出："bab"
解释："aba" 同样是符合题意的答案。
示例 2：

输入：s = "cbbd"
输出："bb"
示例 3：

输入：s = "a"
输出："a"
示例 4：

输入：s = "ac"
输出："a"
 

提示：

1 <= s.length <= 1000
s 仅由数字和英文字母（大写和/或小写）组成

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/longest-palindromic-substring
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

struct Solution{}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // chars返回该字符串的迭代器，使用collect需要显示指定类型
        let seq: Vec<char> = s.chars().collect();  
        let len = seq.len();
        if len < 2 {
            return s
        }
        let (mut idx, mut curr_len, mut curr_start, mut curr_end) = (0,0,0,0);
        while idx < len {
            let (mut i, mut j) = (idx, idx);
            let ch = seq[idx];
            // handle same char
            while i > 0 && seq[i - 1] == ch {
                i -= 1
            }
            while j < len - 1 && seq[j + 1] == ch {
                j += 1
            }
            idx = j + 1;
            while i > 0 && j < len - 1 && seq[i - 1] == seq[j + 1] {
                i -= 1;
                j += 1;
            }
            let max_len = j - i + 1;
            if max_len > curr_len {
                curr_len = max_len;
                curr_start = i;
                curr_end = j;
            }
            if max_len >= len - 1 {
                break;
            }
        }
        s[curr_start..curr_end + 1].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let s = String::from("ababcb");
        assert_eq!("aba".to_string(), Solution::longest_palindrome(s));
    }
}