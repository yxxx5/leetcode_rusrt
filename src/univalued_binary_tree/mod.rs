use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::TreeNode;
use crate::merge_trees::makeRcTree;

pub struct Solution965{}

impl Solution965 {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {

        if root.is_none() {
            return true;
        }
        let t = root.as_ref().unwrap().borrow();
        let val = t.val;
        let left = t.left.clone();
        let right = t.right.clone();

        let mut res = true;

        if left.is_none() && right.is_none() {
            return true;
        }

        if let Some(t) = &left {
            res = res && (val == t.borrow().val);
        }

        if let Some(t) = &right {
            res = res && (val == t.borrow().val);
        }

        if res {
            return Self::is_unival_tree(left.clone()) && Self::is_unival_tree(right.clone());
        } else {
            return  false;
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn exploration(){
        let t1 = makeRcTree(1);
        t1.as_ref().unwrap().borrow_mut().left = makeRcTree(1);
        t1.as_ref().unwrap().borrow_mut().right = makeRcTree(1);
        t1.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow_mut().right = makeRcTree(1);

        let t2 = makeRcTree(1);
        t2.as_ref().unwrap().borrow_mut().left = makeRcTree(1);
        t2.as_ref().unwrap().borrow_mut().right = makeRcTree(1);
        t2.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow_mut().right = makeRcTree(3);

        assert_eq!(Solution965::is_unival_tree(t1), true);
        assert_eq!(Solution965::is_unival_tree(t2), false);
    }

}