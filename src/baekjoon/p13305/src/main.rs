fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let distances = read_line_as_numbers();
    let oil_prices = read_line_as_numbers();
    
    let mut total_cost = 0;
    let mut cur_oil_price = oil_prices[0];
    let mut cur_distance = distances[0];
    for i in 1..oil_prices.len() - 1 {
        if cur_oil_price > oil_prices[i] {
            total_cost += cur_oil_price * cur_distance;
            cur_oil_price = oil_prices[i];
            cur_distance = distances[i];
        } else {
            cur_distance += distances[i];
        }
    }
    total_cost += cur_oil_price * cur_distance;
    println!("{}", total_cost);
}
