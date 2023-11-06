use tree_lib::tree_node::TreeNode;

fn main() {
    let mut tree = TreeNode::new(2);
    tree.insert(2);
    tree.insert(4);
    tree.insert(10);
    tree.insert(5);
    tree.show();
    
}
