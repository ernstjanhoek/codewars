pub struct Node {
    pub value: u32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>
}

impl Node {

    pub fn new(value: u32) -> Self {
        Node {value, left: None, right: None}
    }

    pub fn left(mut self, node: Node) -> Self {
        self.left = self.left.or(Some(Box::new(node)));
        self
    }

    pub fn right(mut self, node: Node) -> Self {
        self.right = self.right.or(Some(Box::new(node)));
        self
    }
}

impl Node {
    fn extract(&self, mut layer: u32) -> Vec<(u32, u32)> {
        let mut out: Vec<(u32, u32)> = Vec::new();
        out.push((self.value, layer));
        layer += 1;
        if let Some(left) = &self.left {
            out.append(&mut left.extract(layer));
        } 
        if let Some(right) = &self.right {
            out.append(&mut right.extract(layer));
        }
        out
    }
}

fn sort(mut vec: Vec<(u32, u32)>) -> Vec<u32> {
    vec.sort_by_key(|k|k.1);
    vec.iter().map(|v|v.0).collect()
}

fn tree_by_levels(root: &Node) -> Vec<u32> {
    let mut out:Vec<(u32, u32)> = Vec::new();
    out.append(&mut root.extract(0u32));
    sort(out)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

// Use the builder pattern to create your own tests:
//   let root = Node::new(1)           // create root
//              .left(Node::new(2))    // chain left child (returns root)
//              .right(Node::new(3));  // chain right child (returns root)

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn root_only() {
        assert_eq!(tree_by_levels(&Node::new(42)),
                   [42],
                   "\nYour result (left) didn't match the expected output (right).");
    }

    #[test]
    fn complete_tree() {
        let root = Node::new(1)
            .left(Node::new(2)
                  .left(Node::new(4))
                  .right(Node::new(5)))
            .right(Node::new(3)
                   .left(Node::new(6)));

        assert_eq!(tree_by_levels(&root),

                   [1,2,3,4,5,6],
                   "\nYour result (left) didn't match the expected output (right).");
    }
}
