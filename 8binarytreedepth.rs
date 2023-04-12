use std::cmp;

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val: val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            cmp::max(left_depth, right_depth) + 1
        }
    }
}

fn main() {
    let mut root = Some(Box::new(TreeNode::new(3)));
    root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(9)));
    root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(20)));
    root.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(15)));
    root.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));

    let depth = max_depth(root);
    println!("The maximum depth of the binary tree is {}", depth);
}
