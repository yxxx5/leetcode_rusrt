pub struct Solution561{}

impl Solution561 {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut v = nums.clone();
        let mut n = 0;
        v.sort();

        for i in 0..v.len()-1 {
          if i % 2 == 0 {
              n += v[i];
          }
        }

        n
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn exploration(){
        use super::*;
        let v = vec![1, 4, 3, 2];
        assert_eq!(Solution561::array_pair_sum(v), 4);
    }
}

