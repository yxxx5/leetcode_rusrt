mod merge_trees;
use crate::merge_trees::{Solution, makeRcTree};

mod next_permutation;
use crate::next_permutation::*;

mod binary_tree_inorder_traversal;
use crate::binary_tree_inorder_traversal::*;

mod single_number;
use crate::single_number::*;

mod kth_smallest_element_in_a_bst;
use crate::kth_smallest_element_in_a_bst::Solution230;

mod num_unique_emails;
use crate::num_unique_emails::Solution929;

mod maximum_depth_of_binary_tree;

mod invert_binary_tree;

mod peak_index_in_a_mountain_array;
use peak_index_in_a_mountain_array::Solution852;

mod self_dividing_numbers;

mod nim_game;

mod number_of_recent_calls;
use number_of_recent_calls::RecentCounter;


fn main() {
    let mut r = RecentCounter::new();
    r.ping(642);
    r.ping(1849);
    let n = r.ping(4921);
    eprintln!("{}", n);
    //r.ping(3003);
    //eprintln!("{}", n);
    //assert_eq!(3, 3);

}
