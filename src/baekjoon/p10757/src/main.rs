const MAX: u64 = 1_000_000_000_000_000_000;
const MAGNITUDE: usize = 18;
fn add_big_numbers(max_len: usize, a: u64, b: u64, carry: u64) -> (u64, String) {
    let sum = a + b + carry;
    let overflow = sum >= MAX;
    if overflow {
        let sum_string = format!("{:01$}", sum - MAX, max_len);
        (1, sum_string)
    } else {
        let sum_string = format!("{:01$}", sum, max_len);
        (0, sum_string)
    }
}

fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|s| s.to_string()).collect()
}


fn get_partial_string(my_string: &mut String) -> (String, usize) {
    let range = if my_string.len() >= MAGNITUDE {
        my_string.len() - MAGNITUDE
    } else {
        0
    };
    let len = if my_string.len() >= MAGNITUDE {
        MAGNITUDE
    } else {
        my_string.len()
    };
    (my_string.drain(range..).collect::<String>(), len)
}

fn main() {
    let mut strings = read_line_as_strings();
    let mut a = strings.remove(0);
    let mut b = strings.remove(0);
    let mut results: Vec<String> = Vec::new();

    let mut carry: u64 = 0;
    while a.len() != 0 || b.len() != 0 {
        let (a_partial, a_len) = get_partial_string(&mut a);
        let (b_partial, b_len) = get_partial_string(&mut b);

        let a_num = a_partial.parse::<u64>().unwrap_or_default();
        let b_num = b_partial.parse::<u64>().unwrap_or_default();
        let (overflow, sum) = add_big_numbers(std::cmp::max(a_len, b_len), a_num, b_num, carry);
        carry = overflow;
        
        results.push(sum);
    }

    if carry != 0 {
        results.push(carry.to_string());
    }

    let result = results
        .iter()
        .rev()
        .map(|s| s.as_str())
        .collect::<String>();

    println!("{}", result);
}
