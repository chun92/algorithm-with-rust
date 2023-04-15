struct Matrix {
    n: u64,
    v: Vec<Vec<u64>>,
}

impl Matrix {
    fn new_for_fibonacchi() -> Matrix {
        Matrix {
            n: 2,
            v: vec![vec![1, 1], vec![1, 0]],
        }
    }

    fn identity(n: u64) -> Matrix {
        let mut v = Vec::new();
        for i in 0..n {
            let mut row = Vec::new();
            for j in 0..n {
                if i == j {
                    row.push(1);
                } else {
                    row.push(0);
                }
            }
            v.push(row);
        }
        Matrix { n, v }
    }

    fn mul(&self, other: &Matrix) -> Matrix {
        let mut v = Vec::new();
        for i in 0..self.n {
            let mut row = Vec::new();
            for j in 0..other.n {
                let mut sum = 0;
                for k in 0..self.n {
                    sum += self.v[i as usize][k as usize] * other.v[k as usize][j as usize] % 1000000007;
                }
                row.push(sum);
            }
            v.push(row);
        }
        Matrix {
            n: self.n,
            v,
        }
    }
}

fn read_line_as_number() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_line_as_number();
    
    let mut m = Matrix::new_for_fibonacchi();
    let mut result = Matrix::identity(2);
    let mut n = n;
    while n > 0 {
        if n % 2 == 1 {
            result = result.mul(&m);
        }
        m = m.mul(&m);
        n /= 2;
    }
    println!("{}", result.v[0][1] % 1000000007);
}
