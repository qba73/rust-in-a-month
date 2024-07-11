fn give_number() -> i32 {
    8
}

fn give_num() -> i32 {
    return 8;
    // unreachable expression!
    //10
}

fn multiply1(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result)
}

fn multiply2(num1: i32, num2: i32) -> i32 {
    let result = num1 * num2;
    result
}

fn main() {
    println!("Hello, world!");

    println!("Hello, world number {}!", give_number());

    println!("{}", give_num());

    multiply1(10, 20);

    let res = multiply2(2, 4);
    println!("result is: {res}")
}
