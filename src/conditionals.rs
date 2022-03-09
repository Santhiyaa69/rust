pub fn conditions() {
    let num:i32 = 17;
    if num%2 == 0 {
    println!("div by 2:{}",num)
    } else if num%3 == 0 {
    println!("div by 3:{}",num)
    } else {
        println!("{0} {1}",num,"is not div by 2 and 3")
    }

    let age:u8 = 18;
    let is_active:bool = true;
    if age >= 18 && is_active{

    } else if age < 18 && is_active{

    } else {

    }

  let number =  if num > 10 { true } else { false };
  println!("{}",number)
}