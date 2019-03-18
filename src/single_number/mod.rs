pub struct Solution136{}

impl Solution136 {
    pub fn single_number(nums: Vec<i32>) -> i32{
        let mut res: i32 = 0;
        for x in nums {
            res ^= x;
        }
        res
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        use super::*;
        let v = vec![1, 1, 2, 2, 3];
        assert_eq!(3, Solution136::single_number(v));
    }
}