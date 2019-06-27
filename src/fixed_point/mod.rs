pub struct Solution1064{}

impl Solution1064 {
    pub fn fixed_point(a: Vec<i32>) -> i32 {
        for (i, &j) in a.iter().enumerate() {
            if (j == i as i32) {
                return j;
            }
        }

        -1
    }
}


#[cfg(test)]
mod tests{
    #[test]
    fn exploration(){
        use super::*;
        let v1 = vec![10,-5,0,3,7];
        let v2 = vec![0,2,5,8,17];
        let v3 = vec![-10,-5,3,4,7,9];
        assert_eq!(Solution1064::fixed_point(v1), 3);
        assert_eq!(Solution1064::fixed_point(v2), 0);
        assert_eq!(Solution1064::fixed_point(v3), -1);
    }
}