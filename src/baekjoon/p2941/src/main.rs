use std::io;

fn read_alphabet() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.trim().chars().peekable();
    let mut count = 0;
    while let Some(c) = iter.next() {
        count += 1;
        match c {
            'c' => {
                if let Some(peek) = iter.peek() {
                    match *peek {
                        '=' => {
                            iter.next().unwrap();
                        },
                        '-' => {
                            iter.next().unwrap();
                        },
                        _ => ()
                    }
                }
            },
            'd' => {
                if let Some(peek) = iter.peek() {
                    match *peek {
                        'z' => {
                            if let Some(peek) = iter.clone().skip(1).next() {
                                match peek {
                                    '=' => {
                                        iter.next().unwrap();
                                        iter.next().unwrap();
                                    },
                                    _ => ()
                                }
                            }
                        },
                        '-' => {
                            iter.next().unwrap();
                        },
                        _ => ()
                    }
                }
            },
            'l' => {
                if let Some(peek) = iter.peek() {
                    match *peek {
                        'j' => {
                            iter.next().unwrap();
                        },
                        _ => ()
                    }
                }
            },
            'n' => {
                if let Some(peek) = iter.peek() {
                    match *peek {
                        'j' => {
                            iter.next().unwrap();
                        },
                        _ => ()
                    }
                }
            },
            's' => {
                if let Some(peek) = iter.peek() {
                    match *peek {
                        '=' => {
                            iter.next().unwrap();
                        },
                        _ => ()
                    }
                }
            },
            'z' => {
                if let Some(peek) = iter.peek() {
                    match *peek {
                        '=' => {
                            iter.next().unwrap();
                        },
                        _ => ()
                    }
                }
            },
            _ => ()
        }
    }
    count
}

fn main() {
    let result = read_alphabet();
    println!("{}", result);
}