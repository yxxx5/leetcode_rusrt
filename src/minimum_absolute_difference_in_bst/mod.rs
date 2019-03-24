use leetcode_prelude::TreeNode;
use std::cmp::min;
use std::rc::Rc;
use std::cell::RefCell;
pub struct Solution530{}
use super::binary_tree_inorder_traversal::Solution94;
use super::makeRcTree;

impl Solution530 {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //中序遍历
        let v = Solution94::inorder_traversal(root);

        let len = v.len();

        let mut n = i32::max_value();

        for i in 0..len - 1 {
            let m = (v[i] - v[i + 1]).abs();

            if m < n {
                n = m;
            }

        }

        n
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn exploration(){
        use super::*;
        let t1 = makeRcTree(1);
        t1.as_ref().unwrap().borrow_mut().left = makeRcTree(3);
        t1.as_ref().unwrap().borrow_mut().right = makeRcTree(2);

        assert_eq!(Solution530::get_minimum_difference(t1), 1);

    }
}

