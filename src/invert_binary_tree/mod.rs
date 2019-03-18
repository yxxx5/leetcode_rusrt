use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::TreeNode;
use super::makeRcTree;



pub struct Solution226{}
impl Solution226 {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut newt: Option<Rc<RefCell<TreeNode>>> = None;

        if let Some(t) = root {
            let val = t.borrow().val.clone();
            let left = t.borrow().left.clone();
            let right = t.borrow().right.clone();

            newt = Some(Rc::new(RefCell::new(TreeNode::new(val))));

            newt.as_ref().unwrap().borrow_mut().left = Self::invert_tree(right);
            newt.as_ref().unwrap().borrow_mut().right = Self::invert_tree(left);
        } else {
            return None;
        }

        newt

    }
}


#[cfg(test)]
mod test {
    #[test]
    fn exploration() {
        use super::*;

        let t1 = makeRcTree(3);
        t1.as_ref().unwrap().borrow_mut().left = makeRcTree(1);
        t1.as_ref().unwrap().borrow_mut().right = makeRcTree(4);
        t1.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow_mut().right = makeRcTree(2);


        Solution226::invert_tree(t1);

    }
}

