struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<i32>,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![0; rows * cols],
        }
    }

    fn get(&self, row: usize, col: usize) -> i32 {
        self.data[row * self.cols + col]
    }

    fn set(&mut self, row: usize, col: usize, value: i32) {
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
                result.set(row, col, sum);
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

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (rows, cols) = {
        let numbers = read_line_as_numbers();
        (numbers[0] as usize, numbers[1] as usize)
    };
    let mut matrix1 = Matrix::new(rows, cols);
    for row in 0..rows {
        let numbers = read_line_as_numbers();
        for col in 0..cols {
            matrix1.set(row, col, numbers[col as usize]);
        }
    }

    let (rows, cols) = {
        let numbers = read_line_as_numbers();
        (numbers[0] as usize, numbers[1] as usize)
    };
    let mut matrix2 = Matrix::new(rows, cols);
    for row in 0..rows {
        let numbers = read_line_as_numbers();
        for col in 0..cols {
            matrix2.set(row, col, numbers[col as usize]);
        }
    }

    let result = matrix1.multiply(&matrix2);
    result.print();
}
