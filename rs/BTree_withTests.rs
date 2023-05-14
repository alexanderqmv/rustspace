struct BinaryTree<T>{ 
    value: T,
    left:  Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,  
}

impl<T> BinaryTree<T> {
    fn new(value: T) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }
    fn left(mut self, node:BinaryTree<T>) -> Self 
    {
        self.left = Some(Box::new(node));
        self 
    }
    fn right(mut self, node:BinaryTree<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}


fn main() {
   
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn create_new_tree() {
        let tree = BinaryTree::new(1);
        assert_eq!(tree.value, 1);
    }
    #[test]
    fn check_left_tree() {
        let left_tree = BinaryTree::new(1).left(BinaryTree::new(2));
        if let Some(node) = left_tree.left {
            assert_eq!(node.value, 2);    
        }
    }

    #[test]
    fn check_right_tree() {
        let right_tree = BinaryTree::new(1).right(BinaryTree::new(2));
        if let Some(node) = right_tree.right {
            assert_eq!(node.value, 2);    
        }
    }
}
 
