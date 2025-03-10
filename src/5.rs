fn main() {
    let x = 5;
    let y = 6;
    println!("The sum of {} and {} is {}", x, y, sum(x, y));
}

fn sum(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}
