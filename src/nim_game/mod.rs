pub struct Solution292{}


/*
    巴什博弈：
    只有一堆n个物品，两个人轮流从这堆物品中取物，规定每次至少取一个，最多取m个。最后取光者得胜。
    如果n=m+1，那无论先手怎么拿都没办法取胜，所以保证留给对手的是m+1就一定会获胜
    所以如果n=k*(m+1) 如果先手的人选择的数是s, 那么后手的人选择的数p 只要保证 s + p == m + 1就能确保获胜。

    所以按照这个策略先留给对手的数是 m+1的倍数的人会获胜
    也就是：n=k*(m+1)后手胜 n%(m+1)==0 先手胜
    变相玩法：
    两个人轮流报数，每次至少报一个，最多报十个，谁能报到100者胜。

 */
impl Solution292{
    pub fn can_win_nim(n: i32) -> bool{
        return n % 4 != 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn exploration(){
        assert_eq!(false, Solution292::can_win_nim(4));
        assert_eq!(false, Solution292::can_win_nim(16));
        assert_eq!(true, Solution292::can_win_nim(19));
    }
}