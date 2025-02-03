fn main() {
    let x = 1;
    println!("The value of x: {}", x);

    let mut y = 5;
    println!("The value of y: {}", y);

    y = 6;
    println!("The value of y: {}", y);

    //Затенение
    let x1 = 5;

    let x1 = x1 + 1;

    {
        let x1 = x1 * 2;
        println!("The number of x1 in the inner scope is: {x1}");
    }

    println!("The value of x1 is: {x1}");
}

