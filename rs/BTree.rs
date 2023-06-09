struct BTree {
    value: i32,
    left: Option<Box<BTree>>,
    right: Option<Box<BTree>>,
}
impl BTree {
    fn new(root: i32) -> Self {
        BTree {
            value: root,
            left: None,
            right: None,
        }
    }
    fn insert(&mut self, val: i32) {
        if val < self.value {
            if let Some(left) = &mut self.left {
                left.insert(val);
            } else {
                self.left = Some(Box::new(BTree::new(val)));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.insert(val);
            } else {
                self.right = Some(Box::new(BTree::new(val)));
            }
        }
    }

    fn print(&self) {
        if let Some(left) = &self.left {
            left.print();
        }
        println!("{}", self.value);

        if let Some(right) = &self.right {
            right.print();
        }
    }
    fn find(&mut self, elem: i32) -> bool {
        if self.value == elem {
            return true;
        }

        if self.value > elem {
            if let Some(left) = &mut self.left {
                return left.find(elem);
            }
        } else {
            if let Some(right) = &mut self.right {
                return right.find(elem);
            }
        }

        return false;
    }
}

fn main() {
    let mut tree = BTree::new(10);
    tree.insert(10);
    tree.insert(1001);
    tree.print();
    let res = tree.find(1001);
    println!("{res}");

}
