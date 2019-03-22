pub struct Solution509{}
impl Solution509 {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else {
            return  Self::fib(n - 1) + Self::fib(n - 2);
        }
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn exploration(){
        use super::*;
        assert_eq!(Solution509::fib(2), 1);
        assert_eq!(Solution509::fib(3), 2);
        assert_eq!(Solution509::fib(4), 3);
    }
}