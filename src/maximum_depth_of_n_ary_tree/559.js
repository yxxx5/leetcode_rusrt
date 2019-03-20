/**
 * // Definition for a Node.
 * function Node(val,children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */
/**
 * @param {Node} root
 * @return {number}
 */
var maxDepth = function(root) {
    if (!root) {
        return 0;
    }
    let n = 1;

    if (root && root.children.length) {
        let v = root.children.map((r)=>{
            return maxDepth(r);
        });
        v.sort();
        return n + v[v.length - 1];
    } else {
        return n;
    }

};