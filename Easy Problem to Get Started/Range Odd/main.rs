fn int_space_input() -> Vec<i32> {
    let mut vec0 = Vec::new();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("input");
    let nums = line.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();
    for num in nums {
        vec0.push(num);
    }
    return vec0;
}

fn find_odd (num1: i32, num2: i32) -> Vec<i32> {
    let mut vec0 = Vec::new();
    for num in num1..num2+1 {
        if num % 2 != 0 {
            vec0.push(num);
        }
    }
    return vec0;
}

fn array_to_space_seperated_string(array: Vec<i32>) -> String {
    let length = array.len();
    let mut answer = array[0].to_string().to_owned();
    answer.push_str(&" ".to_owned());
    for instance in 1..length-1 {
        answer.push_str(&array[instance].to_string().to_owned());
        answer.push_str(&" ".to_owned());
    } 
    answer.push_str(&array[length-1].to_string().to_owned());
    return answer.to_string();
}

fn main() {
    let array = int_space_input();
    let ans = find_odd(array[0], array[1]);
    println!("{}", array_to_space_seperated_string(ans));
}