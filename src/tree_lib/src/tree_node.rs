pub struct TreeNode{
    pub value: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode{
    pub fn new(data: i32) -> TreeNode{
        TreeNode {value: data, left: None, right: None}
    }

    pub fn insert(&mut self, new_value: i32) {
        if new_value < self.value {
            if let Some(left) = &mut self.left {
                left.insert(new_value);
            } else {
                self.left = Some(Box::new(TreeNode::new(new_value)));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.insert(new_value);
            } else {
                self.right = Some(Box::new(TreeNode::new(new_value)));
            }
        }
    }

    pub fn show(&self) {
        if let Some(ref left) = self.left {
            left.show();
        }
 
        println!("{:?}", self.value);
 
        if let Some(ref right) = self.right {
            right.show();
        }
    }
}