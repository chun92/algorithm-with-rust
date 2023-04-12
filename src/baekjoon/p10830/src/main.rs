struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<u64>,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![0; rows * cols],
        }
    }

    fn new_identity(size: usize) -> Matrix {
        let mut result = Matrix::new(size, size);
        for i in 0..size {
            result.set(i, i, 1);
        }
        result
    }

    fn get(&self, row: usize, col: usize) -> u64 {
        self.data[row * self.cols + col]
    }

    fn set(&mut self, row: usize, col: usize, value: u64) {
        self.data[row * self.cols + col] = value;
    }

    fn multiply(&self, other: &Matrix) -> Matrix {
        let mut result = Matrix::new(self.rows, other.cols);
        for row in 0..self.rows {
            for col in 0..other.cols {
                let mut sum = 0;
                for i in 0..self.cols {
                    sum += self.get(row, i) * other.get(i, col);
                }
                result.set(row, col, sum % 1000);
            }
        }
        result
    }

    fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{} ", self.get(row, col));
            }
            println!();
        }
    }
}

fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, b) = {
        let numbers = read_line_as_numbers();
        (numbers[0] as usize, numbers[1] as usize)
    };

    let mut a = Matrix::new(n, n);
    for row in 0..n {
        let numbers = read_line_as_numbers();
        for col in 0..n {
            a.set(row, col, numbers[col]);
        }
    }

    let mut index = b;
    let mut cur = a;
    let mut result = Matrix::new_identity(n);

    while index > 0 {
        if index % 2 == 1 {
            result = result.multiply(&cur);
        }
        cur = cur.multiply(&cur);
        index /= 2;
    }

    result.print();
}
