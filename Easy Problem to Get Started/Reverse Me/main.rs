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

fn int_input() -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().expect("invalid input");
    return n;
}

fn array_to_space_seperated_string(array: &Vec<i32>) -> String {
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
    let _length = int_input();
    let array = int_space_input();
    let mut rev_array: Vec<i32> = Vec::new();
    for x in array.iter().rev() {
        rev_array.push(*x);
    }

    println!("{}", array_to_space_seperated_string(&rev_array));
}