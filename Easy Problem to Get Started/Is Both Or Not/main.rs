fn int_input() -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().expect("invalid input");
    return n;
}

fn is_both_or_not(num: i32) {
    if num % 5 == 0 && num % 11 == 0 {
        println!("BOTH");
    }
    else if num % 5 != 0 && num % 11 != 0 {
        println!("NONE")
    }
    else {
        println!("ONE")
    }
}

fn main() {
    let int_value1: i32 = int_input();
    is_both_or_not(int_value1);
}