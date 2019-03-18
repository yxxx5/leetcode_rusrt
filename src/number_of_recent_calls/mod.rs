use std::cell::RefCell;
use std::rc::Rc;

pub struct RecentCounter {
    v: Rc<RefCell<Vec<i32>>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    pub fn new() -> Self {
        RecentCounter{
            v: Rc::new(RefCell::new(vec![]))
        }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        self.v.borrow_mut().push(t);
        let mut len = self.v.borrow().len();
        let mut i = 0;
        let mut count = 0;
        while i < len {
            if t - self.v.borrow()[i] > 3000 {
                self.v.borrow_mut().remove(i - count);
                len -= 1;
            } else {
                i += 1;
            }
        }

        return self.v.borrow().len() as i32;
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

#[cfg(test)]
mod test{
    #[test]
    fn exploration() {
        use super::*;
        let mut n = 0;
        let  mut r = RecentCounter::new();
        n = r.ping(1);
        assert_eq!(n, 1);
        n = r.ping(100);
        assert_eq!(n, 2);
        n = r.ping(3001);
        assert_eq!(n, 3);
        n = r.ping(3101);
        assert_eq!(n, 2);
    }
}