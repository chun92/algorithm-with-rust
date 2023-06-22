#[derive(Clone, Copy, Debug)]
struct Frame {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

impl Frame {
    fn new(left: usize, right: usize, top: usize, bottom: usize) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

struct Frames {
    frames: Vec<Option<Frame>>,
    count: usize,
    remaining: usize,
    current: String,
    result: Vec<String>,
}

impl Frames {
    fn new() -> Self {
        Self { 
            frames: vec![None; 26],
            count: 0,
            remaining: 0,
            current: String::new(),
            result: Vec::new(),
        }
    }

    fn update_frame(&mut self, character: char, x: usize, y: usize) {
        let index = character as usize - 'A' as usize;
        let frame = self.frames[index].as_mut();

        if let Some(f) = frame {
            f.left = std::cmp::min(f.left, x);
            f.right = std::cmp::max(f.right, x);
            f.top = std::cmp::min(f.top, y);
            f.bottom = std::cmp::max(f.bottom, y);
        } else {
            self.frames[index] = Some(Frame::new(x, x, y, y));
            self.count += 1;
            self.remaining |= 1 << index;
        }
    }

    fn calculate_possible(&mut self, graph: &Vec<Vec<char>>) -> usize {
        let mut possible = 0;
        for i in 0..26 {
            let index = 1 << i;
            if self.remaining & index == 0 {
                continue;
            }
            let frame = self.frames[i].as_ref().unwrap();
            let character = ('A' as u8 + i as u8) as char;
            let left = frame.left;
            let right = frame.right;
            let top = frame.top;
            let bottom = frame.bottom;

            let mut result = true;
            for y in top..=bottom {
                let result1 = graph[y][left] == character || graph[y][left] == '.' ;
                if !result1 {
                    result = false;
                    break;
                }
                let result2 = graph[y][right] == character || graph[y][right] == '.' ;
                if !result2 {
                    result = false;
                    break;
                }
            }

            for x in left..=right {
                let result1 = graph[top][x] == character || graph[top][x] == '.' ;
                if !result1 {
                    result = false;
                    break;
                }
                let result2 = graph[bottom][x] == character || graph[bottom][x] == '.' ;
                if !result2 {
                    result = false;
                    break;
                }
            }

            if result {
                possible |= index;
            }
        }
        possible
    }

    fn remove_frame(&self, index: usize, graph: &mut Vec<Vec<char>>) {
        let frame = self.frames[index].as_ref().unwrap();
        let left = frame.left;
        let right = frame.right;
        let top = frame.top;
        let bottom = frame.bottom;

        for y in top..=bottom {
            graph[y][left] = '.';
            graph[y][right] = '.';
        }

        for x in left..=right {
            graph[top][x] = '.';
            graph[bottom][x] = '.';
        }
    }

    fn solve(&mut self, graph: &Vec<Vec<char>>) {
        if self.current.len() == self.count {
            self.result.push(self.current.clone());
            return;
        }

        let possible = self.calculate_possible(graph);
        for i in 0..26 {
            let index = 1 << i;
            if possible & index == 0 {
                continue;
            }
            let graph = &mut graph.clone();
            self.remove_frame(i, graph);
            self.current.push(('A' as u8 + i as u8) as char);
            self.remaining ^= index;
            self.solve(graph);
            self.remaining |= index;
            self.current.pop();
        }
    }

    fn _print_all(&self) {
        self.frames
        .iter()
        .enumerate()
        .for_each(|(index, frame)| {
            let char = ('A' as u8 + index as u8) as char;
            if let Some(f) = frame {
                println!("{}, {:?}", char, f);
            }
        });
    }
}

fn read_line_as_string_check_eof() -> Option<String> {
    let mut line = String::new();
    let result = std::io::stdin().read_line(&mut line);
    if result.is_err() {
        return None;
    }
    let result = result.unwrap();
    if result == 0 {
        return None;
    }
    Some(line.trim().to_string())
}

fn main() {
        loop {
            let input_start = read_line_as_string_check_eof();
            if input_start.is_none() {
                break;
            }
        let n = input_start.unwrap().parse::<usize>().unwrap();
        let m = read_line_as_string_check_eof().unwrap().parse::<usize>().unwrap();

        let mut frames = Frames::new();
        let mut graph = vec![vec!['.'; m]; n];
        for y in 0..n {
            let line = read_line_as_string_check_eof().unwrap();
            for (x, character) in line.chars().enumerate() {
                if character == '.' {
                    continue;
                }
                frames.update_frame(character, x, y);
                graph[y][x] = character;
            }
        }

        frames.solve(&graph);
        let mut result = frames.result
            .iter()
            .map(|result| result.chars().rev().collect::<String>())
            .collect::<Vec<String>>();
        result.sort();
        for result in result {
            println!("{}", result);
        }
    }
}
