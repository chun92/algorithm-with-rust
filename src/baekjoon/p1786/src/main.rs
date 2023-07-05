fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end_matches('\n').to_string()
}

fn preprocess(target: &Vec<char>) -> Vec<usize> {
    let n = target.len();
    let mut pi = vec![0; n + 1];

    let mut j = 0;
    for i in 1..n {
        while j > 0 && target[i] != target[j] {
            j = pi[j];
        }

        if target[i] == target[j] {
            j += 1;
            pi[i + 1] = j;
        }
    }
    pi
}

fn main() {
    let text = read_line_as_string();
    let finding_string = read_line_as_string();

    let text = text.chars().collect::<Vec<char>>();
    let finding_string = finding_string.chars().collect::<Vec<char>>();

    let pre_table = preprocess(&finding_string);

    let mut indexes = Vec::new();

    let mut j = 0;
    let mut i = 0;
    loop {
        if i >= text.len() {
            break;
        }

        while j > 0 && text[i] != finding_string[j] {
            j = pre_table[j];
        }

        if text[i] == finding_string[j] {
            j += 1;
            if j == finding_string.len() {
                indexes.push(i + 2 - j);
                j = pre_table[j];
            }
            i += 1;
        } else {
            i = if j == 0 { i + 1 } else { i + j - pre_table[j] };
        }
    }

    println!("{}", indexes.len());
    for i in indexes {
        print!("{} ", i);
    }
}
