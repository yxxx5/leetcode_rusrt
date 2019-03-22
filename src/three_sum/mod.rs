/*
    执行用时: 32 ms, 在3Sum的Rust提交中击败了100.00% 的用户
    内存消耗: 1.9 MB, 在3Sum的Rust提交中击败了100.00% 的用户

    O(n2)

    思路：双指针，遍历Vec 取末尾值nums[i] i = len - 1,

    双指针r -> 0 l -> i - 1
    问题就转为 nums[r] + nums[l] == -nums[i]

*/
struct Solution15{

}

impl Solution15 {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut i = nums.len() - 1;
        let mut v: Vec<Vec<i32>> = Vec::new();

        if nums.len() < 3 {
            return v;
        }

        while  i >= 2 {
            let mut r = 0;
            let mut l = i - 1;
            while r < l {
                let sum = nums[r] + nums[l] + nums[i];
                if sum > 0 {
                    l -= 1;
                } else if sum < 0 {
                    r += 1;
                } else {
                    v.push(vec![nums[r], nums[l], nums[i]]);
                    //可以在这里跳过重复数值 末尾可以不需要去重
                    r += 1;
                    l -= 1;
                }
            }
            //跳过重复数值
            loop {
                i -= 1;
                if i < 2 {//这里不加i值限制 nums[i]时会报错rust attempt to subtract with overflow
                    break;
                }
                if nums[i] != nums[i + 1] {
                    break;
                }
            }
        }
        //去重
        v.dedup();

        v
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration(){
        use super::*;
        let v = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(Solution15::three_sum(v), vec![
            [-1, -1, 2],
            [-1, 0, 1]
        ]);
    }
}