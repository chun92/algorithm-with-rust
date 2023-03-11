use std::collections::BTreeSet;

fn find_subsets(set: &BTreeSet<usize>, size: usize) -> Vec<BTreeSet<usize>> {
    let mut subsets = Vec::new();
    let mut indices: Vec<usize> = (0..size).collect();
    loop {
        let mut subset = BTreeSet::new();
        for index in indices.iter() {
            subset.insert(*set.iter().nth(*index).unwrap());
        }
        subsets.push(subset);
        if !increment_indices(&mut indices, set.len()) {
            break;
        }
    }
    subsets
}

fn increment_indices(indices: &mut Vec<usize>, set_size: usize) -> bool {
    let mut i = indices.len() - 1;
    loop {
        let max_index = set_size - (indices.len() - i);
        if indices[i] < max_index {
            indices[i] += 1;
            for j in (i + 1)..indices.len() {
                indices[j] = indices[j - 1] + 1;
            }
            return true;
        }
        if i == 0 {
            return false;
        }
        i -= 1;
    }
}
       

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_score_sum_diff(scores: &Vec<Vec<usize>>, team1: &Vec<usize>, team2: &Vec<usize>) -> usize {
    let mut team1_sum = 0;
    let mut team2_sum = 0;
    
    for i in 0..team1.len() {
        for j in 0..team1.len() {
            team1_sum += scores[team1[i]][team1[j]];
        }
    }
    
    for i in 0..team2.len() {
        for j in 0..team2.len() {
            team2_sum += scores[team2[i]][team2[j]];
        }
    }

    let result = if team1_sum > team2_sum {
        team1_sum - team2_sum
    } else {
        team2_sum - team1_sum
    };

    result
}

fn get_min_diff(scores: &Vec<Vec<usize>>, subsets: &Vec<BTreeSet<usize>>) -> usize {
    let mut min_diff = std::usize::MAX;

    for i in 0..subsets.len() {
        let subset = &subsets[i];
        if subset.contains(&0) {
            let team1: Vec<usize> = (0..scores.len()).filter(|&x| subset.contains(&x)).collect();
            let team2: Vec<usize> = (0..scores.len()).filter(|&x| !subset.contains(&x)).collect();
            
            let min = get_score_sum_diff(scores, &team1, &team2);
            if min < min_diff {
                min_diff = min;
            }
        }
    }

    return min_diff;
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut scores_vec: Vec<Vec<usize>> = Vec::new();
    
    for _ in 0..n {
        let scores = read_line_as_numbers();
        scores_vec.push(scores);
    }
    
    let mut set: BTreeSet<usize> = BTreeSet::new();
    for i in 0..n {
        set.insert(i);
    }

    let subsets = find_subsets(&set, n / 2);

    let result = get_min_diff(&scores_vec, &subsets);
    println!("{}", result);
}