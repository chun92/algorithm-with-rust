use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_number() -> usize {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|s| s.to_string()).collect()
}

fn get_distance(a: usize, b: usize) -> usize {
    let z = a ^ b;
    let mut ans = 0;
    for i in 0..4 {
        if z & (1 << i) != 0 {
            ans += 1;
        }
    }
    ans
}

fn main() {
    let t = read_line_as_number();
    let mut output = BufWriter::new(stdout());
    for _ in 0..t {
        let n = read_line_as_number();
        let strs = read_line_as_strings();
        let mut graph = vec![0; 16];
        for i in 0..n {
            let s = strs[i].clone();
            
            match s.as_str() {
                "ESTJ" => graph[0] += 1,
                "ESTP" => graph[1] += 1,
                "ESFJ" => graph[2] += 1,
                "ESFP" => graph[3] += 1,
                "ENTJ" => graph[4] += 1,
                "ENTP" => graph[5] += 1,
                "ENFJ" => graph[6] += 1,
                "ENFP" => graph[7] += 1,
                "ISTJ" => graph[8] += 1,
                "ISTP" => graph[9] += 1,
                "ISFJ" => graph[10] += 1,
                "ISFP" => graph[11] += 1,
                "INTJ" => graph[12] += 1,
                "INTP" => graph[13] += 1,
                "INFJ" => graph[14] += 1,
                "INFP" => graph[15] += 1,
                _ => panic!("Invalid type"),
            }
        }
        
        let mut flag = false;
        for j in 0..16 {
            if graph[j] >= 3 {
                writeln!(output, "0").unwrap();
                flag = true;
                break;
            }
        }

        if flag {
            continue;
        }

        let mut vec = Vec::new();
        for j in 0..16 {
            if graph[j] == 1 {
                vec.push(j);
            } else if graph[j] == 2 {
                vec.push(j);
                vec.push(j);
            } else if graph[j] >= 3 {
                vec.push(j);
                vec.push(j);
                vec.push(j);
            }
        }

        let mut ans = usize::MAX;
        for x in 0..vec.len() {
            for y in x+1..vec.len() {
                for z in y+1..vec.len() {
                    let a = vec[x];
                    let b = vec[y];
                    let c = vec[z];
                    let d = get_distance(a, b) + get_distance(b, c) + get_distance(c, a);
                    if d < ans {
                        ans = d;
                    }
                }
            }
        }
        writeln!(output, "{}", ans).unwrap();
    }
}
