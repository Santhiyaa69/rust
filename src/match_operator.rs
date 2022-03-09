pub fn run () {
    let input = 78;
    match input {
        0 => println!("no is `0` "),
        1|3|5|7|9 => println!("odd num"),
        2|4|6|8 => println!("Even num"),
        15..=25 => println!("btw 15 to 25"),
        _ => println!("Invalid Input")
    }
}