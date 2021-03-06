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
    let arr1 = int_space_input();
    let arr2 = int_space_input();
    let mut ans = -1;
    for i in 0..arr1[0] {
        if arr2[i as usize] == arr1[1] {
            ans = 1;
        }
    }
    println!("{}", ans);
}