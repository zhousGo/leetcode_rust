/**
 *
 * 给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。
 * 示例 1:
 * 输入: "abcabcbb"
 *
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
 * 示例 2:
 *
 * 输入: "bbbbb"
 * 输出: 1
 * 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
 * 示例 3:
 *
 * 输入: "pwwkew"
 * 输出: 3
 * 释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
 * 请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
 */
fn length_of_longest_substring(s: String) -> i32 {
    use std::cmp::max;
    let mut last: [i32; 128] = [-1; 128];
    let mut left = -1;
    let mut ans = 0;
    for (i, v) in s.chars().enumerate() {
        //
        left = max(left, last[v as usize]);
        last[v as usize] = i as i32;
        ans = max(ans, (i as i32) - left);
    }
    ans
}

#[test]
fn length_of_longest_substring_test() {
    let result = length_of_longest_substring(String::from("abcabcbb"));
    println!("{}", result);
}
