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

fn main() {
    let array = int_space_input();
    if array[0] + array[1] > array[2] && array[1] + array[2] > array[0] && array[0] + array[2] > array[1] {
        println!("YES");
    }
    else {
        println!("NO");
    }
}