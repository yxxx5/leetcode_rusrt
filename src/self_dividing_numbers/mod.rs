pub  struct  Solution728{}
impl Solution728 {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut v = vec![];
        for i in left..right+1 {
            if (i < 10) {
                v.push(i);
            } else if Self::test(i) {
                v.push(i);
            }
        }

        v
    }

    fn test (n: i32) -> bool {
        let mut m = n.clone();
        if n < 10 {
            return true;
        }

        while m > 0 {
            let x = m % 10;

            if x == 0 || n % x != 0 {
                return false;
            }
            m /= 10;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let v = Solution728::self_dividing_numbers(1, 22);
        assert_eq!(v, vec![1,2,3,4,5,6,7,8,9,11,12,15,22]);
    }
}
