

fn main() {
    
    let rand_tuple = ("Suryaprakash", 30);

    let rand_tuple2:(&str, i8) = ("Deepthi", 30);

    println!("Name {}", rand_tuple.0);
    println!("Age {}", rand_tuple.1);
}
