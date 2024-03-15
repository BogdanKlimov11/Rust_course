struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        if value <= self.value {
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

    fn inorder_traversal(&self, result: &mut Vec<i32>) {
        if let Some(ref left) = self.left {
            left.inorder_traversal(result);
        }
        result.push(self.value);
        if let Some(ref right) = self.right {
            right.inorder_traversal(result);
        }
    }
}

fn tree_sort(array: &mut [i32]) {
    let mut root = None;
    
    for &item in array.iter() {
        if let Some(ref mut node) = root {
            node.insert(item);
        } else {
            root = Some(Box::new(Node::new(item)));
        }
    }
    
    let mut result = Vec::new();
    if let Some(ref node) = root {
        node.inorder_traversal(&mut result);
    }
    
    for (i, item) in result.iter().enumerate() {
        array[i] = *item;
    }
}
