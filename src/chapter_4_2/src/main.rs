fn main() {
    //4.2 ссылки и взаимствование
    let s1 = String::from("hello");

    let len: usize = calculate_length(&s1);

    println!("The length of {s1} is {len}");

    //изменяемая ссылка
    let mut s = String::from("hello!!!!");
    change(&mut s);

    println!("Changed s: {s}");
}

fn calculate_length(s1: &String) -> usize {
    s1.len()
}

fn change(s: &mut String) {
    s.push_str(", World!!! from muttable ref");
}

