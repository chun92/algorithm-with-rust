fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_line_as_char() -> Vec<char> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().chars().filter(|&c| c != ' ').collect()
}

struct Tree {
    node: char,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Tree {
    fn new(node: char) -> Self {
        Tree {
            node,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, node: char, left: char, right: char) {
        if self.node == node {
            if left != '.' {
                self.left = Some(Box::new(Tree::new(left)));
            }
            if right != '.' {
                self.right = Some(Box::new(Tree::new(right)));
            }
        } else {
            if let Some(ref mut left_node) = self.left {
                left_node.insert(node, left, right);
            }
            if let Some(ref mut right_node) = self.right {
                right_node.insert(node, left, right);
            }
        }
    }

    fn print_preorder(&self) {
        print!("{}", self.node);
        if let Some(ref left_node) = self.left {
            left_node.print_preorder();
        }
        if let Some(ref right_node) = self.right {
            right_node.print_preorder();
        }
    }

    fn print_inorder(&self) {
        if let Some(ref left_node) = self.left {
            left_node.print_inorder();
        }
        print!("{}", self.node);
        if let Some(ref right_node) = self.right {
            right_node.print_inorder();
        }
    }

    fn print_postorder(&self) {
        if let Some(ref left_node) = self.left {
            left_node.print_postorder();
        }
        if let Some(ref right_node) = self.right {
            right_node.print_postorder();
        }
        print!("{}", self.node);
    }
}


fn main() {
    let n = read_line_as_number();
    let mut tree = Tree::new('A');
    for _ in 0..n {
        let input = read_line_as_char();
        tree.insert(input[0], input[1], input[2]);
    }
    tree.print_preorder();
    println!();
    tree.print_inorder();
    println!();
    tree.print_postorder();
    println!();
}
