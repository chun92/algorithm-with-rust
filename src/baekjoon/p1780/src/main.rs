fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

struct Rect {
    x: usize,
    y: usize, 
    w: usize,
    h: usize
}

impl Rect {
    fn includes(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }
}

struct NinthTree {
    value: Option<i32>,
    position: Rect,
    children: Option<[Box<NinthTree>; 9]>
}

impl NinthTree {
    fn new(x: usize, y: usize, n: usize) -> NinthTree {
        NinthTree {
            value: None,
            position: Rect { x, y, w: n, h: n },
            children: None
        }
    }

    fn merge(&mut self) {
        if self.position.w == 1 {
            return;
        }

        let mut value = None;
        let mut all_same = true;
        for child in self.children.as_mut().unwrap().iter_mut() {
            child.merge();
            if value.is_none() {
                value = child.value;
            } else if value != child.value {
                all_same = false;
            }
        }

        if all_same {
            self.value = value;
            self.children = None;
        }
    }

    fn create_children(&mut self) {
        let w = self.position.w / 3;
        let h = self.position.h / 3;
        let children = [Box::new(NinthTree::new(self.position.x, self.position.y, w)),
                            Box::new(NinthTree::new(self.position.x + w, self.position.y, w)),
                            Box::new(NinthTree::new(self.position.x + 2 * w, self.position.y, w)),
                            Box::new(NinthTree::new(self.position.x, self.position.y + h, w)),
                            Box::new(NinthTree::new(self.position.x + w, self.position.y + h, w)),
                            Box::new(NinthTree::new(self.position.x + 2 * w, self.position.y + h, w)),
                            Box::new(NinthTree::new(self.position.x, self.position.y + 2 * h, w)),
                            Box::new(NinthTree::new(self.position.x + w, self.position.y + 2 * h, w)),
                            Box::new(NinthTree::new(self.position.x + 2 * w, self.position.y + 2 * h, w))];
        self.children = Some(children);
    }

    fn insert(&mut self, x: usize, y: usize, value: i32) {
        if self.position.w == 1 {
            self.value = Some(value);
            return;
        }

        if self.children.is_none() {
            self.create_children();
        }

        for child in self.children.as_mut().unwrap().iter_mut() {
            if child.position.includes(x, y) {
                child.insert(x, y, value);
                return;
            }
        }
    }

    fn count(&self, value: i32) -> usize {
        if self.value.is_some() {
            if self.value.unwrap() == value as i32 {
                return 1;
            } else {
                return 0;
            }
        }

        let mut count = 0;
        for child in self.children.as_ref().unwrap().iter() {
            count += child.count(value);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_includes_1() {
        let rect = Rect { x: 1, y: 1, w: 3, h: 3 };
        assert!(rect.includes(1, 1));
        assert!(rect.includes(1, 3));
        assert!(rect.includes(3, 1));
        assert!(rect.includes(3, 3));

        assert!(!rect.includes(1, 4));
        assert!(!rect.includes(4, 1));
        assert!(!rect.includes(4, 4));
        assert!(!rect.includes(1, 0));
        assert!(!rect.includes(0, 1));
        assert!(!rect.includes(0, 4));
        assert!(!rect.includes(4, 0));
    }

    #[test]
    fn test_rect_includes_2() {
        let rect = Rect { x: 1, y: 1, w: 1, h: 1 };
        assert!(rect.includes(1, 1));
        assert!(!rect.includes(1, 2));
        assert!(!rect.includes(2, 1));
        assert!(!rect.includes(2, 2));
    }

    #[test]
    fn test_tree_1() {
        let mut tree = NinthTree::new(1, 1, 3);
        tree.insert(1, 1, 1);
        tree.insert(1, 2, 1);
        tree.insert(1, 3, 1);
        tree.insert(2, 1, 1);
        tree.insert(2, 2, 1);
        tree.insert(2, 3, 1);
        tree.insert(3, 1, 1);
        tree.insert(3, 2, 1);
        tree.insert(3, 3, 1);
        tree.merge();
        assert_eq!(tree.value, Some(1));
        assert!(tree.children.is_none());
        assert_eq!(tree.count(1), 1);
    }

    
    #[test]
    fn test_tree_2() {
        let mut tree = NinthTree::new(1, 1, 3);
        for i in 1..=3 {
            for j in 1..=3 {
                tree.insert(i, j, 1);
            }
        }
        tree.insert(3, 3, 0);
        tree.merge();
        assert!(tree.value.is_none());
        assert!(tree.children.is_some());
        assert_eq!(tree.count(1), 8);
        assert_eq!(tree.count(0), 1);
    }

    #[test]
    fn test_tree_3() {
        let mut tree = NinthTree::new(1, 1, 9);
        for i in 1..=9 {
            for j in 1..=9 {
                tree.insert(i, j, 1);
            }
        }
        tree.merge();
        assert!(tree.value.is_some());
        assert!(tree.children.is_none());
        assert_eq!(tree.count(1), 1);
    }
    
    #[test]
    fn test_example() {
        let vec = vec![vec![0, 0, 0, 1, 1, 1, -1, -1, -1],
            vec![0, 0, 0, 1, 1, 1, -1, -1, -1],
            vec![0, 0, 0, 1, 1, 1, -1, -1, -1],
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 1, -1, 0, 1, -1, 0, 1, -1],
            vec![0, -1, 1, 0, 1, -1, 0, 1, -1],
            vec![0, 1, -1, 1, 0, -1, 0, 1, -1]];

        let mut tree = NinthTree::new(1, 1, 9);
        for i in 1..=9 {
            for j in 1..=9 {
                tree.insert(i, j, vec[i - 1][j - 1]);
            }
        }
        tree.merge();
        assert_eq!(tree.count(-1), 10);
        assert_eq!(tree.count(0), 12);
        assert_eq!(tree.count(1), 11);
    }

    #[test]
    fn test_result_1() {
        let vec = vec![vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0]];
        let (result1, result2, result3) = get_result(&vec);
        assert_eq!(result1, 1);
        assert_eq!(result2, 0);
        assert_eq!(result3, 0);
    }
    
    #[test]
    fn test_result_2() {
        let vec = vec![vec![1, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 1]];
        let (result1, result2, result3) = get_result(&vec);
        assert_eq!(result1, 0);
        assert_eq!(result2, 1);
        assert_eq!(result3, 0);
    }

    #[test]
    fn test_result_3() {
        let vec = vec![vec![-1, -1, -1],
            vec![-1, -1, -1],
            vec![-1, -1, -1]];
        let (result1, result2, result3) = get_result(&vec);
        assert_eq!(result1, 0);
        assert_eq!(result2, 0);
        assert_eq!(result3, 1);
    }

    
    #[test]
    fn test_result_4() {
        let vec = vec![vec![0, 1, -1],
            vec![0, -1, 1],
            vec![0, 1, -1]];
        let (result1, result2, result3) = get_result(&vec);
        assert_eq!(result1, 3);
        assert_eq!(result2, 3);
        assert_eq!(result3, 3);
    }
    
    #[test]
    fn test_result_5() {
        let vec = vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0]];
        let (result1, result2, result3) = get_result(&vec);
        assert_eq!(result1, 1);
        assert_eq!(result2, 0);
        assert_eq!(result3, 0);
    }

    #[test]
    fn test_result_6() {
        let vec = vec![vec![0, 0, 0, 1, 1, 1, -1, -1, -1],
            vec![0, 0, 0, 1, 1, 1, -1, -1, -1],
            vec![0, 0, 0, 1, 1, 1, -1, -1, -1],
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 1, -1, 0, 1, -1, 0, 1, -1],
            vec![0, -1, 1, 0, 1, -1, 0, 1, -1],
            vec![0, 1, -1, 1, 0, -1, 0, 1, -1]];

        let (result1, result2, result3) = get_result(&vec);
        assert_eq!(result3, 10);
        assert_eq!(result1, 12);
        assert_eq!(result2, 11);
    }
}

fn get_result(vec: &Vec<Vec<i32>>) -> (usize, usize, usize) {
    let mut result1 = 0;
    let mut result2 = 0;
    let mut result3 = 0;
    let result = get_num(0, 0, vec.len(), vec, &mut result1, &mut result2, &mut result3);
    match result {
        Some(0) => result1 += 1,
        Some(1) => result2 += 1,
        Some(-1) => result3 += 1,
        _ => {}
    }
    (result1, result2, result3)
}

fn get_num(x: usize, y: usize, n: usize, vec: &Vec<Vec<i32>>, result1: &mut usize, result2: &mut usize, result3: &mut usize) -> Option<i32> {
    if n == 1 {
        return Some(vec[x][y]);
    }

    let n1 = get_num(x, y, n / 3, vec, result1, result2, result3);
    let n2 = get_num(x + n / 3, y, n / 3, vec, result1, result2, result3);
    let n3 = get_num(x + 2 * n / 3, y, n / 3, vec, result1, result2, result3);
    let n4 = get_num(x, y + n / 3, n / 3, vec, result1, result2, result3);
    let n5 = get_num(x + n / 3, y + n / 3, n / 3, vec, result1, result2, result3);
    let n6 = get_num(x + 2 * n / 3, y + n / 3, n / 3, vec, result1, result2, result3);
    let n7 = get_num(x, y + 2 * n / 3, n / 3, vec, result1, result2, result3);
    let n8 = get_num(x + n / 3, y + 2 * n / 3, n / 3, vec, result1, result2, result3);
    let n9 = get_num(x + 2 * n / 3, y + 2 * n / 3, n / 3, vec, result1, result2, result3);

    if n1.is_some() && n2.is_some() && n3.is_some() && n4.is_some() && n5.is_some() && n6.is_some() && n7.is_some() && n8.is_some() && n9.is_some() {
        let n1 = n1.unwrap();
        let n2 = n2.unwrap();
        let n3 = n3.unwrap();
        let n4 = n4.unwrap();
        let n5 = n5.unwrap();
        let n6 = n6.unwrap();
        let n7 = n7.unwrap();
        let n8 = n8.unwrap();
        let n9 = n9.unwrap();

        if n1 == n2 && n2 == n3 && n3 == n4 && n4 == n5 && n5 == n6 && n6 == n7 && n7 == n8 && n8 == n9 {
            if n1 == 0 {
                return Some(0);
            } else if n1 == 1 {
                return Some(1);
            } else if n1 == -1 {
                return Some(-1);
            } else {
                panic!("error");
            }
        }
    }
    
    let mut add_num = |target: Option<i32>| {
        if target.is_some() {
            let target = target.unwrap();
            if target == 0 {
                *result1 += 1;
            } else if target == 1 {
                *result2 += 1;
            } else if target == -1 {
                *result3 += 1;
            } else {
                panic!("error");
            }
        }
    };
    add_num(n1);
    add_num(n2);
    add_num(n3);
    add_num(n4);
    add_num(n5);
    add_num(n6);
    add_num(n7);
    add_num(n8);
    add_num(n9);
    return None;
}

fn main() {
    let n = read_line_as_numbers()[0] as usize;

    let mut vec = vec![vec![0; n]; n];

    for i in 0..n {
        let line = read_line_as_numbers();
        for j in 0..n {
            vec[i][j] = line[j];
        }
    }

    let (result1, result2, result3) = get_result(&vec);
    println!("{}", result3);
    println!("{}", result1);
    println!("{}", result2);
}
