use std::cmp;


#[derive(Debug)]
struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new() -> Self {
        TreeNode { left: None, right: None }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + cmp::max(left_depth, right_depth)
        }
        None => 0,
    }
}

fn main() {
    
    let mut root = TreeNode::new();
    root.left = Some(Box::new(TreeNode::new()));
    root.right = Some(Box::new(TreeNode {
        left: Some(Box::new(TreeNode::new())),
        right: Some(Box::new(TreeNode::new())),
    }));

    println!("Maximum depth of the binary tree is: {}", max_depth(Some(Box::new(root))));
}
