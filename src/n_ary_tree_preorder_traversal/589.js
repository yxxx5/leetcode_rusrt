/**
 * // Definition for a Node.
 * function Node(val,children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */
/**
 * @param {Node} root
 * @return {number[]}
 */
var preorder = function(root) {
    var res = [];
    if (root) {
        res.push(root.val);
        if (root.children.length) {
            root.children.map((t)=>{
                res = res.concat(preorder(t));
            });
        }
    }


    return res;
};