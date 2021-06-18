fn int_input() -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().expect("invalid input");
    return n;
}

fn rev_star(num: i32){
    let n = num;
    let mut k = n - 1;

    for i in 0..n {
        for _j in 0..k {

            print!(" ");
        }

        k = k - 1;

        for _j in 0..i+1 {
            print!("*");
        }
        println!();
    }
}

fn main() {
    let num = int_input(); 
    rev_star(num)
}