use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_number() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

struct Stars {
    stars : Vec<Vec<char>>
}

impl Stars {
    fn new(n: usize) -> Stars {
        if n == 1 {
            return Stars { stars: vec![vec!['*']] };
        }

        if n == 2 {
            return Stars { stars: vec![
                vec!['*'; 5], 
                vec!['*'], 
                vec!['*', ' ', '*', '*', '*'],
                vec!['*', ' ', '*', ' ', '*'],
                vec!['*', ' ', '*', ' ', '*'],
                vec!['*', ' ', ' ', ' ', '*'],
                vec!['*'; 5], 
            ]};
        }

        let mut stars = Vec::new();
        let w = 4 * n - 3;

        stars.push(vec!['*'; w]);
        stars.push(vec!['*']);

        let prev = Stars::new(n - 1);

        for i in 0..w - 2 {
            let mut row = Vec::new();
            for j in 0..w {
                if j == 0 || j == w - 1 {
                    row.push('*');
                } else if j == 1 || j == w - 2 {
                    if i == 0 && j == w - 2{
                        row.push('*');
                    } else {
                        row.push(' ');
                    }
                } else if i < w - 2 {
                    let target = prev.stars[i].get(j - 2);
                    match target {
                        Some(c) => row.push(*c),
                        None => row.push(' '),
                    }
                }
            }
            stars.push(row);
        }

        let mut row = Vec::new();
        row.push('*');
        row.append(&mut vec![' '; w - 2]);
        row.push('*');
        stars.push(row);
        stars.push(vec!['*'; w]);

        Stars { stars }
    }

    fn print(&self) {
        let mut writer = BufWriter::new(stdout());
        for row in &self.stars {
            for c in row {
                write!(writer, "{}", c).unwrap();
            }
            writeln!(writer).unwrap();
        }
    }
}

fn main() {
    let n = read_line_as_number();
    Stars::new(n).print();
}
