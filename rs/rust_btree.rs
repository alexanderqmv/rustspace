struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T)
        where
            T: std::cmp::Ord,
    {
        if value < self.value {
            if let Some(ref mut left) = self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(Node::new(value)));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(Node::new(value)));
            }
        }
    }

    fn search(&self, value: &T) -> bool
        where
            T: std::cmp::Ord,
    {
        if value == &self.value {
            return true;
        }

        if value < &self.value {
            if let Some(ref left) = self.left {
                return left.search(value);
            } else {
                return false;
            }
        } else {
            if let Some(ref right) = self.right {
                return right.search(value);
            } else {
                return false;
            }
        }
    }
}



fn main() {
    let mut tree = Node::new(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);

    println!("Tree contains 3: {}", tree.search(&3));
    println!("Tree contains 6: {}", tree.search(&6));
}
