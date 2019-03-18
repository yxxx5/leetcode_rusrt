use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::TreeNode;
use std::cmp::max;
use super::makeRcTree;

pub struct Solution104{}
impl Solution104 {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut n= 0;
        if root.is_none() {
            return 0;
        } else {
            let t = root.unwrap();
            n = 1 + max(
                Self::max_depth(t.borrow().left.clone()),
                Self::max_depth(t.borrow().right.clone())
            );
        }
        n
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        use super::*;

        let t1 = makeRcTree(3);
        t1.as_ref().unwrap().borrow_mut().left = makeRcTree(1);
        t1.as_ref().unwrap().borrow_mut().right = makeRcTree(4);
        t1.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow_mut().right = makeRcTree(2);


        assert_eq!(Solution104::max_depth(t1), 3);
    }
}