pub struct Solution821{}

impl Solution821 {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut v: Vec<i32> = vec![];
        let v1:Vec<char> = s.chars().collect();
        let len = v1.len();
        let mut res: Vec<i32> = vec![];
        
        for i in 0..len {
            if v1[i] == c {
                v.push(i as i32);
            }
        }

        for i in 0..len {
            let mut n = len as i32;
            for j in &v {
                if (i as i32 == *j) {
                    n = 0;
                } else {
                    let m: i32 = (*j  - i as i32).abs();
                    if m < n {
                        n = m;
                    }
                }
            }
            res.push(n);
        }

        res
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn exploration(){
        use super::*;
        assert_eq!(Solution821::shortest_to_char(String::from("loveleetcode"), 'e'),
        vec![3,2,1,0,1,0,0,1,2,2,1,0]);

    }
}