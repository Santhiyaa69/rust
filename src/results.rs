fn divide(a: u32, b: u32) -> Result<u32, String> {
    if b != 0 {
      Ok(a / b)
    } else {
      Err("Division by zero!".to_string())
    }
  }

  pub fn run() {
    let res = divide(6,2);
    println!("{:?}",res);
}