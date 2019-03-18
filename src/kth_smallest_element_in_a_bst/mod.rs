use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::TreeNode;
use super::makeRcTree;
/*
    1.
    if 左叶长度 == k 或 k -1的时候可以确定返回值为max(leftnode) 或 root val

    if 左叶长度 > k
        kth_smallest(leftnode, k)
    if 左叶长度 < k
        kth_smallest(right, k - 左叶长度 - 1)

    2.中序遍历的第k个元素
*/
pub struct Solution230{}

impl Solution230 {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {

        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();
        let leftLen = treeLen(left.clone());

        if leftLen == k {
            return max(left);
        } else if leftLen + 1 == k {
            return root.unwrap().borrow().val;
        }


        if  leftLen > k {
            return Self::kth_smallest(left.clone(), k);
        } else {
            return Self::kth_smallest(right.clone(), k - leftLen - 1);
        }
    }
}

fn treeLen(root: Option<Rc<RefCell<TreeNode>>>) -> i32{
    let mut len = 1;

    if root.is_none() {
        return 0;
    }

    len += treeLen(root.as_ref().unwrap().borrow().left.clone()) + treeLen(root.as_ref().unwrap().borrow().right.clone());

    len
}

fn max(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let right = root.as_ref().unwrap().borrow().right.clone();
    if right.is_none() {
        return root.unwrap().borrow().val;
    } else {
        return  max(right.clone());
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
        assert_eq!(Solution230::kth_smallest(t1.clone(), 2), 2);
    }
}

