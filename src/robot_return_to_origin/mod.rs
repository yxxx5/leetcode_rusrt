pub struct Solution657{}
impl Solution657 {
    pub fn judge_circle(moves: String) -> bool {
        let mut a: (i32,i32) = (0, 0);

        for c in moves.chars() {
            if c == 'U' {
                a.0 += 1;
            } else if c == 'D'{
                a.0 -= 1;
            } else if c == 'L' {
                a.1 += 1;
            } else if c == 'R' {
                a.1 -= 1;
            }
        }

        if a == (0, 0) {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn exploration(){
        use super::*;

        let s1 = String::from("UD");
        let s2 = String::from("LL");

        assert_eq!(Solution657::judge_circle(s1), true);
        assert_eq!(Solution657::judge_circle(s2), false);

    }
}