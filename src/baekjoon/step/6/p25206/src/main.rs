use std::io;

fn read_line_and_calculate_score() -> (f64, f64) {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>();
    
    let score = value[1].parse::<f64>().unwrap();
    let grade = value[2].to_string();

    let result = get_score(grade);
    if result == -1.0 {
        (0.0, 0.0)
    } else {
        (score, score * result)
    }
}

fn get_score(grade:String) -> f64 {
    match grade.as_str() {
        "A+" => 4.5,
        "A0" => 4.0,
        "B+" => 3.5,
        "B0" => 3.0,
        "C+" => 2.5,
        "C0" => 2.0,
        "D+" => 1.5,
        "D0" => 1.0,
        "F" => 0.0,
        _  => -1.0,
    }
}

fn main() {
    let mut score_sum = 0.0;
    let mut grade_sum = 0.0;
    for _ in 0..20 {
        let (score, grade) = read_line_and_calculate_score();
        score_sum += score;
        grade_sum += grade;
    }

    println!("{}", grade_sum / score_sum);
}
