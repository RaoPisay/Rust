
fn main() {
    let rand_array = [1,2,3];
    println!("{}", rand_array[0]);
    println!("length => {}", rand_array.len());
    println!("Second 2 : {:?}", &rand_array[1..3]);
        // Meaning starting from 1 to 3; if you give beyond available index then it won't work
        // passing '&' means passing reference  
}