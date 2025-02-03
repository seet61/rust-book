fn main() {
    //4.1
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    
    println!("{s1}, world!");
    println!("s1 = {s1}, s2 = {s2}");

    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    let s = String::from("hello");

    takes_ownership(s);

    //Нельзя. Не копируется значение при вызове функции. Только копируется сслыка на кучу. 
    //println!("{s}");

    let x = 5;

    make_copy(x);

    println!("{x}");

    let str1 = String::from("hello");
    let (str2, len) = calculate_length(str1);

    println!("The length of '{str2}' is '{len}'");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn make_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn calculate_length(s1: String) -> (String, usize) {
    let length = s1.len();
    (s1, length)
}