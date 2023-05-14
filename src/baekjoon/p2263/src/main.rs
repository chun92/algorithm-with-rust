fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

struct Tree {
    node: usize,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Tree {
    fn print_preorder(&self) {
        print!("{} ", self.node);
        if let Some(ref left) = self.left {
            left.print_preorder();
        }
        if let Some(ref right) = self.right {
            right.print_preorder();
        }
    }
}

fn build_preorder(inorder: &[usize], postorder: &[usize]) -> Tree {
    let root = postorder[postorder.len() - 1];
    let root_inorder = inorder.iter().position(|&x| x == root).unwrap();
    let left_inorder = &inorder[..root_inorder];
    let right_inorder = &inorder[root_inorder + 1..];
    let left_postorder = &postorder[..left_inorder.len()];
    let right_postorder = &postorder[left_inorder.len()..postorder.len() - 1];
    Tree {
        node: root,
        left: if left_inorder.len() > 0 {
            Some(Box::new(build_preorder(left_inorder, left_postorder)))
        } else {
            None
        },
        right: if right_inorder.len() > 0 {
            Some(Box::new(build_preorder(right_inorder, right_postorder)))
        } else {
            None
        },
    }
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let inorder = read_line_as_numbers();
    let postorder = read_line_as_numbers();

    let tree = build_preorder(&inorder, &postorder);
    tree.print_preorder();
    println!();
}
