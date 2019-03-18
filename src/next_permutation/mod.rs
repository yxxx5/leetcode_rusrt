pub struct Solution31 {}
impl Solution31 {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut r = 0;
        let mut l = 0;
        /*
            双指针 l、r, l是能在右侧找到大于nums[l]值的最大索引值，r是索引l右侧比nums[l]大且差值最小值的索引
        */
        for (i, n) in nums.iter().enumerate() {
            let num = nums[i];
            let mut bMin = 0;
            for j  in i+1..len {
                if nums[j] > num && (bMin == 0 || nums[j] < bMin) {
                    l = i;
                    r = j;
                    bMin = nums[j];
                }
            }

        }
        //交换r和l值
        if r != 0 {
            let m = nums[r] ^ nums[l];
            nums[r] = nums[r] ^ m;
            nums[l] = nums[l] ^ m;
        }

        //l 右侧部分从小到大排序
        if r == 0 && l == 0 {
            nums.sort();
        } else {
            nums[l+1..].sort();
        }

    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        use super::*;
        let mut v = vec![1, 1,3, 2];
        let mut v1 = vec![3, 2, 1];
        Solution31::next_permutation(&mut v);
        Solution31::next_permutation(&mut v1);
        assert_eq!(v, vec![1, 2, 1, 3]);
        assert_eq!(v1, vec![1, 2, 3]);
    }
}