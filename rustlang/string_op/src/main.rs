

fn main() {
    let mut string_one = "I'm a random string";
    println!("{}", string_one);
    println!("Length : {}", string_one.len());
    let (first, second) = string_one.split_at(6);
    println!("First => {}\nSecond => {}", first, second);

    let char_count = string_one.chars().count();
    let mut chars = string_one.chars();

    let mut individual_char = chars.next();
    loop {
        match individual_char {
            Some(x) => println!("{}", x),
            None => break,
        }
        individual_char = chars.next();        
    }

    let mut iter = string_one.split_whitespace();
    let mut indi_word = iter.next();

    loop {
        match indi_word {
            Some(x) => println!("{}",x),
            None => break,
        }
        indi_word = iter.next();
    }

    //Splitting individual lines using 
    let random_string = "I'm line #1\nI'm Line #2\nI'm Line #3";
    let mut lines = random_string.lines();
    let mut line = lines.next();
    loop {
        match line {
            Some(x) => println!("{}", x),
            None => break,
        }
        line = lines.next();
    }
    println!("Contains #3? {}", random_string.contains("#3")); 
}
