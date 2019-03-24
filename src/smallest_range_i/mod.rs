pub struct Solution908{}

impl Solution908 {
    pub fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
        let mut v = a.clone();
        v.sort();

        let len = v.len();
        let max = v[len - 1];
        let min = v[0];

        if (max - k <= min + k) {
            return 0;
        } else {
            return max - min - 2 * k;
        }
    }
}
#[cfg(test)]
mod test{
    #[test]
    fn exploration(){
        use super::*;

        let A = vec![0, 10];
        let K = 2;
        assert_eq!(Solution908::smallest_range_i(A, K), 6);

    }
}