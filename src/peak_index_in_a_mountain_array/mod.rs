pub struct Solution852{}
impl Solution852 {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut r = 0;
        let mut l = a.len();
        let mut mid = 0;

        while r < l {
            mid = (r + l) / 2;
            if a[mid - 1] < a[mid] {
                if a[mid] > a[mid + 1] {
                    return mid as i32;
                }
                r = mid
            } else {
                l = mid;
            }
        }


        mid as i32
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn exploration() {
        use super::*;
        let v = vec![0, 1, 0];
        let n = Solution852::peak_index_in_mountain_array(v);

        assert_eq!(n, 1);
    }
}