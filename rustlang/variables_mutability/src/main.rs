fn main() {

    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6; // not allowed because x is not mutable variable, put mut before x and try.
    println!("The value of x now is {}", x);

    //constants
    const MILLION: u32 = 100_00_00;
    const PI: f32 = 3.14;
    

    //Note : const and mut can't coexists;
    println!("1 MILLION is denoted by {}", MILLION);
    println!("Pi value is {}", PI);
    println!("Million is {0}, Pi value is {1}", MILLION, PI);

    //You cannot assign the value x again once declared but you can shodow it by using let x again 

    let y = 30;
    let y = "Suryaprakash";
    println!("My Name is {0} and I'm {0} old", y);

    //One more example of shadowing

    let spaces = "      ";
    let spaces = spaces.len();
    println!("Spaces len is {}", spaces);

}
