fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().unwrap()
}

struct Star {
    stars: Vec<Vec<char>>,
}

impl Star {
    fn new(n: usize) -> Star {
        let mut stars = Vec::new();
        if n == 1 {
            let mut row = Vec::new();
            row.push('*');
            stars.push(row);
            return Star {
                stars: stars,
            };
        } 

        let prev = Star::new(n - 1);
        let height = 2_usize.pow(n as u32) - 1;
        let width = height * 2 - 1;

        if n % 2 == 0 {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push('*');
            }
            stars.push(row);
            for i in 0..(height / 2) {
                let mut row = Vec::new();
                for _ in 0..=i {
                    row.push(' ');
                }
                row.push('*');
                for _ in 0..(width / 4 - (i + 1)) {
                    row.push(' ');
                }
                for j in 0..prev.stars[i].len() {
                    row.push(prev.stars[i][j]);
                }
                for _ in 0..(width / 2 - (i + 1) * 2) {
                    row.push(' ');
                }
                row.push('*');
                stars.push(row);
            }
            for i in 0..(height / 2) {
                let mut row = Vec::new();
                for _ in 0..(width / 4 + (i + 1) ) {
                    row.push(' ');
                }
                row.push('*');
                if i == (height / 2 - 1) {
                    stars.push(row);
                    continue;
                }
                for _ in 0..((width / 4 - (i + 1)) * 2 - 1) {
                    row.push(' ');
                }
                row.push('*');
                stars.push(row);
            }
        } else if n % 2 == 1 {
            for i in (0..(height / 2)).rev() {
                let mut row = Vec::new();
                for _ in 0..(width / 4 + (i + 1) ) {
                    row.push(' ');
                }
                row.push('*');
                if i == (height / 2 - 1) {
                    stars.push(row);
                    continue;
                }
                for _ in 0..((width / 4 - (i + 1)) * 2 - 1) {
                    row.push(' ');
                }
                row.push('*');
                stars.push(row);
            }
            for i in (0..(height / 2)).rev() {
                let mut row = Vec::new();
                for _ in 0..=i {
                    row.push(' ');
                }
                row.push('*');
                for _ in 0..(width / 4 - (i + 1)) {
                    row.push(' ');
                }
                for j in 0..prev.stars[(height / 2) - (i + 1)].len() {
                    row.push(prev.stars[(height / 2) - (i + 1)][j]);
                }
                for _ in 0..(width / 2 - (i + 1) * 2) {
                    row.push(' ');
                }
                row.push('*');
                stars.push(row);
            }
            let mut row = Vec::new();
            for _ in 0..width {
                row.push('*');
            }
            stars.push(row);
        }

        Star {
            stars: stars,
        }
    }

    fn print(&self) {
        for i in 0..self.stars.len() {
            for j in 0..self.stars[i].len() {
                print!("{}", self.stars[i][j]);
            }
            println!("");
        }
    }
}

fn main() {
    let n = read_line_as_number();
    let star = Star::new(n);
    star.print();
}
