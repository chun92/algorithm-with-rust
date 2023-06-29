
#[derive(Clone, Debug)]
struct SquareMatrix {
    size: usize,
    data: Vec<Vec<u64>>,
}

const MOD : u64 = 1_000_000_007;

impl SquareMatrix {
    fn new(n: usize) -> Self {
        SquareMatrix { 
            size: n, 
            data: vec![vec![0; n]; n],
        }
    }

    fn set(&mut self, i: usize, j: usize, value: u64) {
        self.data[i][j] = value;
    }
    
    fn get(&self, i: usize, j: usize) -> u64 {
        self.data[i][j]
    }

    fn multifly(&self, other: &Self) -> Self {
        // assert_eq!(self.size, other.size);\
        for i in 0..self.size {
            for j in 0..self.size {
                for k in 0..self.size {
                    result.data[i][j] = ((self.data[i][k] % MOD) * (other.data[k][j] % MOD) % MOD + result.data[i][j]) % MOD;
                }
            }
        }
        result
    }

    fn pow(&self, mut n: u64) -> Self {
        let mut result = Self::new(self.size);
        let mut current = self.clone();
        for i in 0..self.size {
            result.data[i][i] = 1;
        }
        while n > 0 {
            if n & 1 == 1 {
                result = result.multifly(&current);
            }
            current = current.multifly(&current);
            n >>= 1;
        }
        result
    }
}

fn read_line_as_numbers() -> Vec<usize> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("read error");
    s.split_whitespace().map(|e| e.parse().unwrap()).collect()
}
fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut matrix = SquareMatrix::new(n);

    for _ in 0..m {
        let (a, b) = {
            let v = read_line_as_numbers();
            (v[0] - 1, v[1] - 1)
        };
        matrix.set(a, b, 1);
        matrix.set(b, a, 1);
    }

    let d = {
        let v = read_line_as_numbers();
        v[0]
    };

    let result = matrix.pow(d as u64);
    println!("{}", result.get(0, 0));
}
