/*
    双指针
    右指针为初始位置 如果发现重复字符 更新至重复字符
    左指针字符串索引从1递增至字符串长度

    注意 “ ” 长度为1的字符串res为1

    执行用时: 8 ms, 在Longest Substring Without Repeating Characters的 Rust提交中击败了80.88% 的用户
    内存消耗: 843.8 KB, 在Longest Substring Without Repeating Characters的 Rust提交中击败了83.33% 的用户

*/

pub struct Solution3{}

impl Solution3 {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let len = s.len();
        if (len == 1) {
            return 1;
        }
        let mut r = 0;
        let mut l = 1;
        let mut res = 0;
        let mut c = "";
        while l < len {
            l += 1;
            c = &s[(l - 1)..l];
            //发现重复字符 更新起始位置右指针到重复字符的位置
            if let Some(n) = s[r..(l-1)].find(c) {
                r = r + n + 1;
            }
            if l - r > res {
                res = l - r;
            }
        }


        return res as i32;
    }
}
#[cfg(test)]
mod test {
    #[test]
    fn exploration(){
        use super::*;
        assert_eq!(Solution3::length_of_longest_substring(String::from("abcabcbb")), 3);
        assert_eq!(Solution3::length_of_longest_substring(String::from("bbbbb")), 1);
        assert_eq!(Solution3::length_of_longest_substring(String::from("pwwkew")), 3);
    }
}
