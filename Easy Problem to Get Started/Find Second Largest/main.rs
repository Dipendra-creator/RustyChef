fn int_input() -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().expect("invalid input");
    return n;
}

fn main() {
    let mut vec0 = Vec::new();
    for _instance in 0..3 {
        let element = int_input();
        vec0.push(element);
    }
    vec0.sort();
    println!("{}", vec0[1]);
}