pub struct Solution169{}

impl Solution169 {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut res:i32 = 0;
        let mut cnt:i32 = 0;;

        for x in nums {
            if (cnt == 0) {
                res = x;
                cnt +=1;
            } else {
                if (res == x) {
                    cnt += 1;
                } else {
                    cnt -= 1;
                }
            }
        }

        return res;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn exploration(){
        use super::*;
        let v1 = vec![3, 2, 3];
        let v2 = vec![2,2,1,1,1,2,2];
        assert_eq!(Solution169::majority_element(v1), 3);
        assert_eq!(Solution169::majority_element(v2), 2);
    }
}