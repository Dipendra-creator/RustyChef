fn int_input() -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().expect("invalid input");
    return n;
}

fn factor_list(num: i32) -> String {
    if num > 1 {
        let mut factors = "1".to_owned();
        let mut count = 1;
        factors.push_str(&" ".to_owned());
        for x in 2..num {
            if num % x == 0 {
                count = count + 1;
                factors.push_str(&x.to_string().to_owned());
                factors.push_str(&" ".to_owned())
            }
        }
        factors.push_str(&num.to_string().to_owned());
        println!("{}",count+1);
        return factors.to_string();
    }
    println!("{}", num);
    return num.to_string();    
}

fn main() {
    let int_value1: i32 = int_input();
    let array = factor_list(int_value1);
    print!("{}", array)
}