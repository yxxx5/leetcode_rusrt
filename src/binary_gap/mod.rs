pub struct Solution868{}

impl Solution868 {
    pub fn binary_gap(n: i32) -> i32 {
        let mut m = 1;
        let mut res = 1;

        for i in 0..32 {
            if (n>>i) & 1 ==1 {
                if (m > res) {
                    res = m;
                }
                m = 1;
            } else {
                m += 1;
            }
        }


        res
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn exploration(){
        use super::*;

        assert_eq!(Solution868::binary_gap(22), 2);
        assert_eq!(Solution868::binary_gap(6), 2);
        assert_eq!(Solution868::binary_gap(0b100001), 5);

    }
}