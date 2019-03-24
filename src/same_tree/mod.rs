use leetcode_prelude::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
pub struct Solution146{}

impl Solution146 {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        } else if p.is_some() && q.is_some() {
            let pc = p.as_ref().unwrap();
            let qc = q.as_ref().unwrap();
            if pc.borrow().val == qc.borrow().val {
                return Self::is_same_tree(
                    pc.borrow().left.clone(), qc.borrow().left.clone()
                ) && Self::is_same_tree(
                    pc.borrow().right.clone(), qc.borrow().right.clone()
                );
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn exploration(){
        use super::*;
        use super::super::makeRcTree;

        let t1 = makeRcTree(1);
        t1.as_ref().unwrap().borrow_mut().left = makeRcTree(3);
        t1.as_ref().unwrap().borrow_mut().right = makeRcTree(2);

        let t2 = makeRcTree(1);
        t2.as_ref().unwrap().borrow_mut().left = makeRcTree(3);
        t2.as_ref().unwrap().borrow_mut().right = makeRcTree(2);

        assert_eq!(Solution146::is_same_tree(t1, t2), true);

    }
}