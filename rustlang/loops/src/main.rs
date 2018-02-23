fn main() {

    let mut num = 1;
    loop {
        if num%2 == 0 {
            println!("{}", num);
            num += 1;
            continue;
        }
        if num >= 10 {
            break;
        }
        num += 1;
    }

    let mut another_num = 10;
    while another_num >= 0 {
        println!("{}", another_num);
        another_num -= 1;
    }

    for z in 10..20 {
        println!("For {} ", z);
    }
}
