use std::io;

fn read_line_and_print() {
    loop {
        let mut s = String::new();
        let bytes_read = io::stdin().read_line(&mut s).unwrap();
        if bytes_read != 0 {
            println!("{}", s.trim());
        } else {
            break;
        }
    }
}

fn main() {
    read_line_and_print();
}