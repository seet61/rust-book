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

        if counter == 10 {
            break counter * 2; 
        }
    };

    println!("The result of loop is {result}");

    println!("loop names");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count ==2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    println!("While example");
    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");

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
