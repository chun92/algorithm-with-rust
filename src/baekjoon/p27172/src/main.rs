fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn print_vec(numbers: &Vec<usize>, vec: &Vec<i32>) {
    let result = numbers
        .iter()
        .map(|x| vec[*x].to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", result);
}

fn main() {
    let n = read_line_as_numbers()[0];
    let numbers = read_line_as_numbers();
    let mut results = vec![0; 1000001];
    let mut exist = vec![false; 1000001];


    for i in 0..n {
        exist[numbers[i]] = true;
    }

    for i in 0..n {
        let num = numbers[i];
        let mut temp = num;
        while temp <= 1000000 {
            if exist[temp] {
                results[temp] -= 1;
                results[num] += 1;
            }
            temp += num;
        }
        
    }

    print_vec(&numbers, &results);
}
