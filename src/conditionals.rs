pub fn conditions() {
    let num:i32 = 17;
    if num%2 == 0 {
    println!("div by 2:{}",num)
    } else if num%3 == 0 {
    println!("div by 3:{}",num)
    } else {
        println!("{0} {1}",num,"is not div by 2 and 3")
    }


  let number =  num > 10;
  println!("{}",number)
}