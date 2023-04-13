use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_number() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().unwrap()
}

struct RectStar {
    stars: Vec<Vec<char>>,
}

impl RectStar {
    fn new(n: usize) -> Self {
        let mut stars = Vec::new();
        if n == 1 {
            stars.push(vec!['*']);
            return Self { stars };
        }

        let length = 4 * n - 3;
        let border = vec!['*'; length];
        let mut inner_border = vec![' '; length];
        inner_border[0] = '*';
        inner_border[length - 1] = '*';

        stars.push(border.clone());
        stars.push(inner_border.clone());
        let prev = Self::new(n - 1);
        for i in 0..(length - 4) {
            let mut row = Vec::new();
            row.push('*');
            row.push(' ');
            row.append(&mut prev.stars[i].clone());
            row.push(' ');
            row.push('*');
            stars.push(row);
        }
        stars.push(inner_border.clone());
        stars.push(border.clone());
        Self { stars }
    }

    fn print(&self) {
        let mut output = BufWriter::new(stdout());
        for row in &self.stars {
            for star in row {
                write!(output, "{}", star).unwrap();
            }
            writeln!(output).unwrap();
        }
    }
}

fn main() {
    let n = read_line_as_number();
    let rect_star = RectStar::new(n);
    rect_star.print();
}
