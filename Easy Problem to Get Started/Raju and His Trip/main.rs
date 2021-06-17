fn int_input() -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().expect("invalid input");
    return n;
}

fn can_take_bus(num: i32) {
    if num % 5 != 0 && num % 6 != 0 {
        println!("NO");
    }
    else {
        println!("YES")
    }
}

fn main() {
    let int_value1: i32 = int_input();
    can_take_bus(int_value1);
}