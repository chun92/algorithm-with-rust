fn read_line_as_u32s() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn main() {
    let args = read_line_as_u32s();
    let (n, m) = (args[0], args[1]);
    let mut nums = read_line_as_u32s();
    nums.sort();
    let mut cur = 0;
    for i in 0..n {
        if nums[i as usize] > m {
            break;
        }
        for j in i + 1..n {
            if nums[i as usize] + nums[j as usize] > m {
                break;
            }
            for k in j + 1..n {
                let result = nums[i as usize] + nums[j as usize] + nums[k as usize];
                if result > m {
                    break;
                }
                if result > cur {
                    cur = result;
                }
            }
        }
    }
    println!("{}", cur);
}
