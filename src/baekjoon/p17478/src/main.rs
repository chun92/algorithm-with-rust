use std::io::{stdin, stdout, Write, BufWriter, Stdout};

fn read_line_as_number() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

fn print(depth: usize, output: &mut BufWriter<Stdout>, str: &str) {
    for _ in 0..depth {
        write!(output, "____").expect("Failed to write");
    }
    writeln!(output, "{}", str).expect("Failed to write");
}

fn recursive(n: usize, depth: usize, output: &mut BufWriter<Stdout>) {
    if n == 0 {
        print(depth, output, "\"재귀함수가 뭔가요?\"");
        print(depth, output, "\"재귀함수는 자기 자신을 호출하는 함수라네\"");
        print(depth, output, "라고 답변하였지.");
        return;
    }

    print(depth, output, "\"재귀함수가 뭔가요?\"");
    print(depth, output, "\"잘 들어보게. 옛날옛날 한 산 꼭대기에 이세상 모든 지식을 통달한 선인이 있었어.");
    print(depth, output, "마을 사람들은 모두 그 선인에게 수많은 질문을 했고, 모두 지혜롭게 대답해 주었지.");
    print(depth, output, "그의 답은 대부분 옳았다고 하네. 그런데 어느 날, 그 선인에게 한 선비가 찾아와서 물었어.\"");
    recursive(n - 1, depth + 1, output);
    print(depth, output, "라고 답변하였지.");
}

fn main() {
    let mut output = BufWriter::new(stdout());
    let n = read_line_as_number();
    print(0, &mut output, "어느 한 컴퓨터공학과 학생이 유명한 교수님을 찾아가 물었다.");
    recursive(n, 0, &mut output);
}
