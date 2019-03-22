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
var postorder = function(root) {
    let res = [];
    if (!root) {
        return []
    }

    res.push(root.val);
    if (root.children.length) {
        return root.children.map((r)=>{
            return postorder(r);
        }).reduce((a, b)=>{
            return a.concat(b)
        }, []).concat(res);
    } else {
        return res;
    }
};