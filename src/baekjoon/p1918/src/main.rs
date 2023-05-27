use std::collections::VecDeque;

fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

struct Tree {
    value: char,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Tree {
    fn new(value: char) -> Tree {
        Tree {
            value,
            left: None,
            right: None,
        }
    }

    fn print_postorder(&self) {
        if let Some(left) = &self.left {
            left.print_postorder();
        }
        if let Some(right) = &self.right {
            right.print_postorder();
        }
        print!("{}", self.value);
    }
}

fn make_tree(iter: &mut std::str::Chars) -> Tree {
    let mut stack = Vec::new();
    let mut cur_tree: Option<Tree> = None;
    while let Some(c) = iter.next() {
        match c {
            'A'..='Z' => {
                let tree = Tree::new(c);
                if let Some(mut cur) = cur_tree {
                    cur.right = Some(Box::new(tree));
                    stack.push(cur);
                    cur_tree = None;
                } else {
                    stack.push(tree);
                }
            }
            '+' | '-' => {
                let tree = Tree::new(c);
                stack.push(tree);
            }
            '*' | '/' => {
                let mut tree = Tree::new(c);
                let left = stack.pop().unwrap();
                tree.left = Some(Box::new(left));
                cur_tree = Some(tree);
            }
            '(' => {
                let tree = make_tree(iter);
                if let Some(mut cur) = cur_tree {
                    cur.right = Some(Box::new(tree));
                    stack.push(cur);
                    cur_tree = None;
                } else {
                    stack.push(tree);
                }
            }
            ')' => {
                break;
            }
            _ => unreachable!()
        }
    }

    let mut queue = stack.into_iter().collect::<VecDeque<_>>();
    let mut root = queue.pop_front().unwrap();
    while let Some(mut cur) = queue.pop_front() {
        if (cur.value == '+' || cur.value == '-') && cur.left.is_none() && cur.right.is_none() {
            cur.left = Some(Box::new(root));
            root = cur;
        } else {
            root.right = Some(Box::new(cur));
        }
    }
    root
}

fn main() {
    let str = read_line_as_string();
    
    let mut iter = str.chars();
    let tree = make_tree(&mut iter);
    tree.print_postorder();
}

