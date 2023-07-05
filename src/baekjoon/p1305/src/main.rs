fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_line_as_string() -> Vec<char> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().chars().collect()
}

fn get_pi(n: usize, s: Vec<char>) -> Vec<usize> {
    let mut pi = vec![0; n + 1];
    let mut j = 0;

    for i in 1..n {
        while j > 0 && s[i] != s[j] {
            j = pi[j];
        }

        if s[i] == s[j] {
            j += 1;
            pi[i + 1] = j;
        }
    }

    pi
}

fn main() {
    let n = read_line_as_number();
    let s = read_line_as_string();
    let pi = get_pi(n, s);

    let max = n - pi[n];
    let possible = pi[max];
    let mut remain = max;

    while possible > remain {
        remain = remain - possible;
        if remain == possible {
            break;
        }

        if pi[remain] != possible {
            break;
        }
    }

    if possible == remain {
        println!("{}", possible);
    } else {
        println!("{}", max);
    }
}
