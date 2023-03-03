fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn merge_sort(vec: &mut Vec<i32>, p: usize, r: usize, count: &mut u32, k: u32) {
    if p >= r {
        return;
    }

    let q = (p + r) / 2;
    merge_sort(vec, p, q, count, k);
    merge_sort(vec, q + 1, r, count, k);
    merge(vec, p, q, r, count, k);
}

fn merge(vec: &mut Vec<i32>, p: usize, q: usize, r: usize, count: &mut u32, k: u32) {
    let mut i = p;
    let mut j = q + 1;
    let mut tmp = Vec::new();

    while i <= q && j <= r {
        if vec[i] <= vec[j] {
            tmp.push(vec[i]);
            i += 1;
        } else {
            tmp.push(vec[j]);
            j += 1;
        }
    }

    while i <= q {
        tmp.push(vec[i]);
        i += 1;
    }

    while j <= r {
        tmp.push(vec[j]);
        j += 1;
    }

    i = p;
    while i <= r {
        vec[i] = tmp[i - p];
        *count += 1;
        if *count == k {
            println!("{}", vec[i]);
            std::process::exit(0);
        }
        i += 1;
    }
}

fn main() {
    let arguments = read_line_as_numbers();
    let (n, k) = (arguments[0], arguments[1]);
    let mut vec = read_line_as_numbers();
    let mut count = 0;
    merge_sort(&mut vec, 0, n as usize - 1, &mut count, k as u32);
    println!("-1");
}
