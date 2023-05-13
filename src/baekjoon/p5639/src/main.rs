use std::io::{stdin, stdout, Write, BufWriter, Stdout};

fn read_line_as_number() -> Option<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().ok()
}

struct BST {
    node: usize,
    left: Option<Box<BST>>,
    right: Option<Box<BST>>,
}

impl BST {
    fn new(node: usize) -> Self {
        BST {
            node,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, node: usize) {
        if node < self.node {
            match &mut self.left {
                Some(left) => left.insert(node),
                None => self.left = Some(Box::new(BST::new(node))),
            }
        } else {
            match &mut self.right {
                Some(right) => right.insert(node),
                None => self.right = Some(Box::new(BST::new(node))),
            }
        }
    }

    fn postorder(&self, output: &mut BufWriter<Stdout>) {
        if let Some(left) = &self.left {
            left.postorder(output);
        }
        if let Some(right) = &self.right {
            right.postorder(output);
        }
        writeln!(output, "{}", self.node).unwrap();
    }
}

fn main() {
    let root = read_line_as_number();
    let mut bst = BST::new(root.unwrap());
    while let Some(node) = read_line_as_number() {
        bst.insert(node);
    }

    let mut output = BufWriter::new(stdout());

    bst.postorder(&mut output);
}
