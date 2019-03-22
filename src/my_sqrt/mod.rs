
/*
    x的平方根
    题目归类在双指针：通过双指针 二分查找 找到n x == n * n
*/
struct Solution69 {}
impl Solution69 {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1 {
            return x;
        }
        let mut left = 0;
        let mut right = x;
        while (left < right) {
            let mid = left + (right - left) / 2;
            if x / mid == mid {
                return mid;
            } else if x / mid < mid {
                right = mid;
            } else {
                left = mid + 1
            }
        }
        /*
            这里right需要 - 1 是根据提交来看是要找到第一个不大于x平方的数
            这里的right是最后一个mid mid大于目标值 所以需要 -1
        */
        return right - 1;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn exploration(){
        use super::*;

        assert_eq!(Solution69::my_sqrt(4), 2);
        assert_eq!(Solution69::my_sqrt(8), 2);
    }
}

