struct Triangle {
    stars: Vec<Vec<char>>,
}

impl Triangle {
    fn new(n: usize) -> Self {
        if n == 3 {
            Self {
                stars: vec![
                    vec![' ', ' ', '*', ' ', ' '],
                    vec![' ', '*', ' ', '*', ' '],
                    vec!['*', '*', '*', '*', '*'],
                ]
            }
        } else {
            let triangle = Self::new(n / 2);
            let mut stars = Vec::new();
            for i in 0..(n / 2) {
                let mut row = Vec::new();
                for _ in 0..(n / 2) {
                    row.push(' ');
                }
                for j in 0..(n - 1) {
                    row.push(triangle.stars[i][j]);
                }
                for _ in 0..(n / 2) {
                    row.push(' ');
                }
                stars.push(row);
            }
            for i in 0..(n / 2) {
                let mut row = Vec::new();
                for j in 0..(n - 1) {
                    row.push(triangle.stars[i][j]);
                }
                row.push(' ');
                for j in 0..(n - 1) {
                    row.push(triangle.stars[i][j]);
                }
                stars.push(row);
            }
            Self { stars }
        }
    }

    fn print(&self) {
        self.stars
            .iter()
            .for_each(|row| println!("{}", row.iter().collect::<String>()));
    }
}

fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let triangle = Triangle::new(n);
    triangle.print();
}
