use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_chars() -> Vec<char> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect()
}

fn main() {
    let (l, c) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut chars = read_line_as_chars();
    chars.sort();

    let mut output = BufWriter::new(stdout());
    let mut indexes = (0..l).collect::<Vec<_>>();

    loop {
        let mut vowels = 0;
        let mut consonants = 0;
        let mut str = String::new();
        for i in 0..l {
            if chars[indexes[i]] == 'a' || chars[indexes[i]] == 'e' || chars[indexes[i]] == 'i' || chars[indexes[i]] == 'o' || chars[indexes[i]] == 'u' {
                vowels += 1;
            } else {
                consonants += 1;
            }
            str.push(chars[indexes[i]]);
        }

        if vowels >= 1 && consonants >= 2 {
            writeln!(output, "{}", str).unwrap();
        }

        if indexes[0] == c - l {
            break;
        }

        let mut i = 0;
        while i < l {
            if indexes[l - 1 - i] != c - 1 - i {
                for j in 0..=i {
                    indexes[l - 1 - j] = indexes[l - 1 - i] + i - j + 1;
                }
                break;
            }
            i += 1;
        }
    }
}