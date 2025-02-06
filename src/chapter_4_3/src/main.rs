fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("{word}");

    s.clear();

    //println!("the first word is: {word}"); error after clear. 

    let my_string = String::from("hello world!!!!! !!!!");

    //Тут вызываем как срез строки
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    //Либо как ссылка на строку
    let word = first_word(&my_string);

    //Тут вызываем как строковыйлитерал
    let my_string_literal = "hello world!!!!! man!!!!";

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    //но можно и просто передать в явном виде
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
