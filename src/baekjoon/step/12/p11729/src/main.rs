use std::io;
use io::Write;

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn get_other_tower(i: usize, j: usize) -> usize {
    let mut k = 0;
    for x in 1..=3 {
        if x != i && x != j {
            k = x;
        }
    }
    k
}

fn move_hanoi_tower(out: &mut io::BufWriter<io::Stdout>, n: usize, i: usize, j: usize) {
    if n == 1 {
        writeln!(out, "{} {}", i, j).unwrap();
    } else {
        move_hanoi_tower(out, n - 1, i, get_other_tower(i, j));
        move_hanoi_tower(out, 1, i, j);
        move_hanoi_tower(out, n - 1, get_other_tower(i, j), j);
    }
}

fn main() {
    let n = read_line_as_number();
    let mut out = io::BufWriter::new(io::stdout());
    writeln!(out, "{}", 2i32.pow(n as u32) - 1).unwrap();
    move_hanoi_tower(&mut out, n as usize, 1, 3);
}
