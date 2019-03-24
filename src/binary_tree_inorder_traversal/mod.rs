use leetcode_prelude::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
use super::makeRcTree;

pub struct Solution94{}
impl Solution94 {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut v = Vec::new();
        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();
        if left.is_some() {
            v.extend(Self::inorder_traversal(left));
        }
        v.push(root.unwrap().borrow().val);
        if right.is_some() {
            v.extend(Self::inorder_traversal(right));
        }
        v
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn exploration() {
        use super::*;

        let t1 = makeRcTree(1);
        t1.as_ref().unwrap().borrow_mut().left = makeRcTree(3);
        t1.as_ref().unwrap().borrow_mut().right = makeRcTree(2);

        Solution94::inorder_traversal(t1);

    }
}