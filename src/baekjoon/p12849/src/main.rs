
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
        // assert_eq!(self.size, other.size);
        let mut result = Self::new(self.size);
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

fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please type a number")
}

fn main() {
    let n = read_line_as_number();
    let mut matrix = SquareMatrix::new(8);
    // 0: 정보과학관
    // 1: 전산관
    // 2: 미래관
    // 3: 신양관
    // 4: 한경직기념관
    // 5: 진리관
    // 6: 학생회관
    // 7: 형남공학관

    matrix.set(0, 1, 1);
    matrix.set(1, 0, 1);

    matrix.set(0, 2, 1);
    matrix.set(2, 0, 1);

    matrix.set(1, 2, 1);
    matrix.set(2, 1, 1);

    matrix.set(1, 3, 1);
    matrix.set(3, 1, 1);

    matrix.set(2, 3, 1);
    matrix.set(3, 2, 1);

    matrix.set(2, 4, 1);
    matrix.set(4, 2, 1);

    matrix.set(3, 4, 1);
    matrix.set(4, 3, 1);

    matrix.set(3, 5, 1);
    matrix.set(5, 3, 1);

    matrix.set(4, 5, 1);
    matrix.set(5, 4, 1);

    matrix.set(5, 6, 1);
    matrix.set(6, 5, 1);

    matrix.set(4, 7, 1);
    matrix.set(7, 4, 1);

    matrix.set(6, 7, 1);
    matrix.set(7, 6, 1);

    let result = matrix.pow(n as u64);
    println!("{}", result.get(0, 0));
}
