use leetcode_prelude::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution{}
impl Solution {
    pub fn merge_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let t : Option<Rc<RefCell<TreeNode>>> = makeRcTree(0);

        let mut t1Left = None;
        let mut t1Right = None;
        let mut t2Left = None;
        let mut t2Right = None;

        //let mut val = 0;
        //如果两个树都为None 返回None
        if !t1.is_some() && !t2.is_some() {
            return None;
        }

        if t1.is_some() && t2.is_some() {
            t.as_ref().unwrap().borrow_mut().val = t1.as_ref().unwrap().borrow().val + t2.as_ref().unwrap().borrow().val;

        }  else if t1.is_some() {
            t.as_ref().unwrap().borrow_mut().val = t1.as_ref().unwrap().borrow().val;

        } else if t2.is_some() {
            t.as_ref().unwrap().borrow_mut().val = t2.as_ref().unwrap().borrow().val;

        }

        if t1.is_some() {
            t1Left = t1.clone().unwrap().borrow().left.clone();
            t1Right = t1.clone().unwrap().borrow().right.clone();
        }

        if t2.is_some() {
            t2Left = t2.clone().unwrap().borrow().left.clone();
            t2Right = t2.clone().unwrap().borrow().right.clone();
        }

        t.as_ref().unwrap().borrow_mut().left = Self::merge_trees(t1Left, t2Left);
        t.as_ref().unwrap().borrow_mut().right = Self::merge_trees(t1Right, t2Right);
        return t;
    }
}

pub fn makeRcTree(val:i32) -> Option<Rc<RefCell<TreeNode>>> {
    return Some(Rc::new(RefCell::new(TreeNode::new(val))));
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        use super::*;

        let t1 = makeRcTree(1);
        t1.as_ref().unwrap().borrow_mut().left = makeRcTree(3);
        t1.as_ref().unwrap().borrow_mut().right = makeRcTree(2);

        let t2 = makeRcTree(2);
        t2.as_ref().unwrap().borrow_mut().left = makeRcTree(1);
        t2.as_ref().unwrap().borrow_mut().right = makeRcTree(3);

        let t3 = Solution::merge_trees(t1, t2);
    }
}