pub struct Solution344{}
impl Solution344 {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        if len == 0 {
            return;
        }

        for i in 0..len/2 {
            let mut temp = s[i];
            s[i] = s[len - i - 1];
            s[len - i - 1] = temp;
        }
    }
}
#[cfg(test)]
mod tests {
    fn exploration(){
        use super::*;

        let mut v = vec!['h','e','l','l','o'];
        let v1 = vec!['o','l','l','e','h'];
        Solution344::reverse_string(&mut v);
        assert_eq!(v , v1);

    }
}