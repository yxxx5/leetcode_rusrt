pub struct Solution557{}
impl Solution557 {
    pub fn reverse_words(s: String) -> String {
        let v:Vec<&str> = s.as_str().split(' ').collect();
        let mut res = String::from("");

        for str in v {

            res = res + &str.chars().rev().collect::<String>() + " ";
        }

        res = res.trim_right().to_string();

        res
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn exploration(){
        use super::*;

        let s = String::from("Let's take LeetCode contest");
        assert_eq!(Solution557::reverse_words(s), String::from("s'teL ekat edoCteeL tsetnoc"));

    }
}

