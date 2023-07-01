fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn make_map() -> u128 {
    let mut map = 0;
    for i in 0..10 {
        let chars = read_line_as_string().chars().collect::<Vec<char>>();
        for j in 0..10 {
            if chars[j] == 'O' {
                map |= 1 << (i * 10 + j);
            }
        }
    }
    map
}

fn check_all(map: u128) -> u8 {
    let mut answer = u8::MAX;

    for bit in 0..1_u128 << 10 {
        let mut map = map;
        let mut count = 0;

        for i in 0..10 {
            if bit & (1 << i) != 0 {
                count += 1;
                map ^= 1 << i;
                if i > 0 {
                    map ^= 1 << (i - 1);
                }
                if i < 9 {
                    map ^= 1 << (i + 1);
                }
                map ^= 1 << (10 + i);
            }
        }
        let mut current_row = map & 0b1111111111;

        for i in 1..10 {
            for j in 0..10 {
                if current_row & (1 << j) != 0 {
                    count += 1;
                    map ^= 1 << (i * 10 + j);
                    map ^= 1 << ((i - 1) * 10 + j);
                    if i < 9 {
                        map ^= 1 << ((i + 1) * 10 + j);
                    }
                    if j > 0 {
                        map ^= 1 << (i * 10 + j - 1);
                    }
                    if j < 9 {
                        map ^= 1 << (i * 10 + j + 1);
                    }
                }
            }
            current_row = (map & (0b1111111111 << (i * 10))) >> (i * 10);
        }

        if map == 0 {
            answer = answer.min(count);
        }
    }

    answer
}

fn main() {
    let map = make_map();
    let answer = check_all(map);
    println!("{}", if answer == u8::MAX { -1 } else { answer as i32 });
}
