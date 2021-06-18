fn int_input() -> i64 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: i64 = line.trim().parse().expect("invalid input");
    return n;
}

fn sum_odds(num: i64) -> i64 {
    return num*num;
}

fn sum_evens(num: i64) -> i64 {
    return num*(num+1);
}

fn main() {
    let num = int_input();
    print!("{} ", sum_odds(num));
    println!("{}", sum_evens(num));
}
