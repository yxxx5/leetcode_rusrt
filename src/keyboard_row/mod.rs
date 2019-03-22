pub struct Solution500{}
impl Solution500{
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut v: Vec<String> = vec![];

        for s in words {
            if Self::matched(&s) {
                v.push(s.clone());
            }
        }

        v

    }

    fn matched(s: &String) -> bool {
        let vec = vec![
            vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
            vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
            vec!['z', 'x', 'c', 'v', 'b', 'n', 'm']
        ];
        let mut v1:Vec<char> = vec![];
        let str = s.to_lowercase();

        let char = str.chars().next();

        for v in vec {
            if v.contains(&char.unwrap()) {
                v1 = v.clone();
            }
        }

        for c in str.chars() {
            if !v1.contains(&c) {
                return false;
            }
        }


        return true;
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn exploration(){
        use super::*;

        let v = vec![String::from("Hello"), String::from("Alaska"),
                     String::from("Dad"), String::from("Peace")];
        let v1 = Solution500::find_words(v);

        assert_eq!(v1, vec![
        String::from("Alaska"),
        String::from("Dad")]);
    }
}