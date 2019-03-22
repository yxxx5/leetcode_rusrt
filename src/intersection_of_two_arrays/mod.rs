/*
    两个数的交集

    思路：
    排序两个数组 初始化两个索引值 i1 i2，遍历如果索引对应值相等加入结果数组 两个索引值都+1
    否则对nums1[i1] nums2[i2]中较小值的索引+1
*/
pub struct Solution349{}
impl Solution349 {
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut i1 = 0;
        let mut i2 = 0;

        nums1.sort();
        nums2.sort();

        while i1 < nums1.len() && i2 < nums2.len() {
            if (nums1[i1] == nums2[i2]) {
                res.push(nums1[i1]);
                i1 += 1;
                i2 += 1;
            } else if nums1[i1] > nums2[i2] {
                i2 += 1;
            } else {
                i1 += 1;
            }
        }

        res.dedup();

        return res;
    }
}


#[cfg(test)]
mod tests{
    #[test]
    fn exploration(){
        use super::*;
        let mut v1 = vec![1, 2, 2, 1];
        let mut v2 = vec![2, 2];
        let mut v3 = vec![4, 9, 5];
        let mut v4 = vec![9, 4, 9, 8, 4];


        assert_eq!(Solution349::intersection(v1, v2), vec![2]);
        assert_eq!(Solution349::intersection(v3, v4), vec![4, 9]);
    }
}
