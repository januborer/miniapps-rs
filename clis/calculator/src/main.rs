use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first: String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    let first_num = first.parse::<f64>().unwrap();
    let second_num = second.parse::<f64>().unwrap();
    let result = operate(operator, first_num, second_num);

    println!("{}", output(first_num, operator, second_num, result));
}

fn operate(operator: char, first_num: f64, second_num: f64) -> f64 {
    match operator {
        '+' => first_num + second_num,
        '-' => first_num - second_num,
        '/' => first_num / second_num,
        '*' | 'x' | 'X' => first_num * second_num,
        _ => panic!("Invalid opreator used."),
    }
}

fn output(first_num: f64, operator: char, second_num: f64, result: f64) -> String {
    format!("{} {} {} = {}", first_num, operator, second_num, result)
}
