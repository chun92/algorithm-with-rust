fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, m) = {
        let args = read_line_as_numbers();
        (args[0], args[1])
    };

    let mut vec = vec![Vec::new(); 101];
    let mut memo = vec![0; 101];

    for i in 1..101 {
        for j in 1..=6 {
            if i + j <= 100 {
                vec[i].push(i + j);
            }
        }
    }

    let mut ladder = vec![0; 101]; 

    for _ in 0..n + m {
        let (x, y) = {
            let args = read_line_as_numbers();
            (args[0], args[1])
        };
        ladder[x] = y;
    }

    let mut queue = std::collections::VecDeque::new();
    queue.push_back((1, 0));

    while !queue.is_empty() {
        let mut next_queue = std::collections::VecDeque::new();
        while let Some((pos, cnt)) = queue.pop_front() {
            if pos == 100 {
                println!("{}", cnt);
                return;
            }
            if memo[pos] != 0 {
                continue;
            }

            if ladder[pos] != 0 {
                queue.push_back((ladder[pos], cnt));
                continue;
            }
            
            memo[pos] = cnt;
            for &next in &vec[pos] {
                if memo[next] == 0 {
                    next_queue.push_back((next, cnt + 1));
                }
            }
        }
        queue = next_queue;
    }
}
