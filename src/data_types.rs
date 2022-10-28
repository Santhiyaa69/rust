// use std::char::decode_utf16;
pub fn types() {
    let num1 = 10.5;
    let num2 = 5.5;
    println!("{}",num1-num2);

    let num3:f32 = 10.5;
    let num4:f32 = 5.5;
    println!("{}",num3+num4);

    let name = "Santhiyaa";
    println!("{}",name);

    let language: &str = "Rust";
    println!("{}",language);

    let text = String::from("data");
    println!("{}",text);

    let is_float = true;
    println!("{}",is_float);
    
    let is_greater = 20<10;
    println!("is_Greater:{}",is_greater);

    let  a = 'A';
    println!("character_Literal:{}",a);

    let unicode = '\u{20B9}';
    println!("Unicode:{}",unicode);


    // println!("Max i32:{}",std::i32::MAX);
    // println!("Min i32:{}",std::i32::MIN);

}