use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::TreeNode;
use crate::merge_trees::makeRcTree;


pub struct Solution700{}
impl Solution700 {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_some() {
            let rv = root.as_ref().unwrap().borrow().val;
            if val > rv {
                return Self::search_bst(root.as_ref().unwrap().borrow().right.clone(), val);
            } else if val < rv {
                return Self::search_bst(root.as_ref().unwrap().borrow().left.clone(), val);
            } else {
                return root;
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    #[test]

    fn exploration(){
        use super::*;

        let t1 = makeRcTree(4);
        t1.as_ref().unwrap().borrow_mut().left = makeRcTree(2);
        t1.as_ref().unwrap().borrow_mut().right = makeRcTree(7);

        t1.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = makeRcTree(1);
        t1.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().right = makeRcTree(3);


        let t = Solution700::search_bst(t1, 2);


        let v = aa(t);
        assert_eq!(v, vec![2, 1, 3]);

        fn aa(root:Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut v:Vec<i32> = vec![];
            let tree = root.as_ref().unwrap().borrow();
            let val = tree.val.clone();
            let left = tree.left.clone();
            let right = tree.right.clone();

            if root.is_some() {
                v.push(val);

                if left.is_some() {
                    v.extend(aa(left));
                }

                if right.is_some() {
                    v.extend(aa(right));
                }

            }

            v
        }

    }
}