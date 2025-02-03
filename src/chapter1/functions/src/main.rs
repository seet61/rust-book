use std::result;

fn main() {
    println!("Hello, world from main!");

    another_functin(5);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x: {x}");

    let x = plus_one(5);
    println!("The value of x: {x}");

    let condition = true;

    let number = if condition {5} else {6};

    println!("The value of number is: {number}");

    // это счетчик для примера цикла, поэтому изменяемая переменная
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter = 10 {
            break counter * 2; 
        }
    };
}

fn another_functin(x: i32) {
    println!("Another function! With x: {x}");
}

fn print_labeled_measurement(value: i32, unit_lable: char) {
    println!("The meaasurement is: {value}{unit_lable}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
