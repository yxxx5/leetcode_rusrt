struct Solution942 {}
/*
    IDID 0 4 1 3 2
    DDI 3 2 0 1

    [0, 1, ..., N]
    I的位置是0开始 再遇到I +1
    D的位置是N开始 再遇到I -1
*/
impl Solution942 {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut left = 0;
        let mut right = s.chars().count() as i32;
        let mut v: Vec<i32> = vec![];

        for c in s.chars() {
            if c == 'I' {
                v.push(left);
                left += 1;
            } else {
                v.push(right);
                right -= 1;
            }
        }

        v.push(left);

        v
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn exploration(){
        use super::*;

        let s = "IDID".to_string();
        assert_eq!(Solution942::di_string_match(s), vec![0, 4, 1, 3, 2]);
    }
}
