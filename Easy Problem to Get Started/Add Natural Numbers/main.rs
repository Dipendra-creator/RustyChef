fn int_input() -> i64 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: i64 = line.trim().parse().expect("invalid input");
    return n;
}

fn main() {
    let num = int_input();
    let mut sum: i64 = 0;
    for i in 1..num+1 {
        sum = sum + i;
    }
    println!("{}", sum);
}