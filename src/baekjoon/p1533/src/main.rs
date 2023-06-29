const MOD: u64 = 1_000_003;

#[derive(Clone)]
struct SquareMatrix {
    data: Vec<Vec<u64>>,
    size: usize,
}

impl SquareMatrix {
    fn new(n: usize) -> Self {
        let mut data = vec![vec![0; 5 * n]; 5 * n];

        for i in 0..n {
            data[i * 5 + 0][i * 5 + 1] = 1;
            data[i * 5 + 1][i * 5 + 2] = 1;
            data[i * 5 + 2][i * 5 + 3] = 1;
            data[i * 5 + 3][i * 5 + 4] = 1;
        }
        Self { data, size: n }
    }

    fn get(&self, i: usize, j: usize) -> u64 {
        self.data[i * 5][j * 5]
    }

    fn set(&mut self, i: usize, j: usize, val: u64) {
        match val {
            0 => {},
            1 => {
                self.data[i * 5 + 0][j * 5] = 1;
            },
            2 => {
                self.data[i * 5 + 1][j * 5] = 1;
            },
            3 => {
                self.data[i * 5 + 2][j * 5] = 1;
            },
            4 => {
                self.data[i * 5 + 3][j * 5] = 1;
            },
            5 => {
                self.data[i * 5 + 4][j * 5] = 1;
            },
            _ => panic!("Invalid value"),
        }
    }

    fn multifly(&self, other: &Self) -> Self {
        let mut data = vec![vec![0; 5 * self.size]; 5 * self.size];
        for i in 0..5 * self.size {
            for j in 0..5 * self.size {
                for k in 0..5 * self.size {
                    data[i][j] = (self.data[i][k] * other.data[k][j] + data[i][j]) % MOD;
                }
            }
        }
        Self {
            data,
            size: self.size,
        }
    }

    fn identity(n: usize) -> Self {
        let mut data = vec![vec![0; 5 * n]; 5 * n];
        for i in 0..5 * n {
            data[i][i] = 1;
        }
        Self { data, size: n }
    }

    fn pow(&self, n: u64) -> Self {
        let mut result = Self::identity(self.size);
        let mut base = self.clone();
        let mut n = n;
        while n > 0 {
            if n & 1 == 1 {
                result = result.multifly(&base);
            }
            base = base.multifly(&base);
            n >>= 1;
        }
        result
    }

    fn _print(&self) {
        for i in 0..5 * self.size {
            for j in 0..5 * self.size {
                print!("{} ", self.data[i][j]);
            }
            println!();
        }
    }
}

fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_line_as_numbers_from_string() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect()
}

fn main() {
    let (n, s, e, t) = {
        let v = read_line_as_numbers();
        (v[0] as usize, v[1] as usize - 1, v[2] as usize - 1, v[3])
    };
    let mut matrix = SquareMatrix::new(n);
    for i in 0..n {
        let v = read_line_as_numbers_from_string();
        for j in 0..n {
            matrix.set(i, j, v[j]);
        }
    }

    let result = matrix.pow(t).get(s, e);
    println!("{}", result);
}
