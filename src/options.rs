fn divide(numerator:i32, denominator: i32) -> Option<i32> {
    if denominator == 0 {
        None
    } else {
        Some(numerator/denominator)
    }
}


pub fn run() {
   let result =  divide(10,0);
   match result {
       Some(val) => println!("result is:{}", val),
       None => println!("cannot divide by 0")
   }
}