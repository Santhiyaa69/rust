fn divide(a: u32, b: u32) -> Result<u32, String> {
    if b != 0 {
      Ok(a / b)
    } else {
      Err("Division by zero!".to_string())
    }
  }

  pub fn run() {
    let res = divide(6,2);
    // println!("{:?}",res);

    // let data = vec!["A", "B", "C"];
    // let result = data
    //     .iter()
    //     .map(|x| x.to_string())
    //     .intersperse("->".to_string())
    //     .collect::<String>();
    // println!("{}", result);

    let w1 = Some("demo".to_string());
    let w2 = Some("dummy".to_string());
    let w3 = Some("doodle".to_string());
    let v = vec![w1,w2,w3];

    // println!("{:?}", v);

//convert option to result
    let before: Option<i32> = Some(-2);
    let after: Result<i32, &str> = before.ok_or_else(|| "no value");
    // println!( "{:?}",after.map_err(|x| x.to_string()));

//convert result to option
    let before: Result<i32, &str> = Ok(-2);
    let after: Option<i32> = before.ok();
    // println!("{:?}", after);

    let before: Result<i32, &str> = Err("no value");
    let after: Option<i32> = before.ok();
    // println!("{:?}", after);

//and-then
    let before= Ok(2);
    let after = before.and_then(|x| {
      if x > 0 {
        Ok(x.to_string())
      } else {
        Err("value gt than 0")
      }
    });
    // println!("{:?}", after);

    let bef = Some(5);
    let aft = bef.and_then(|x| {
      if x > 0 {
        Some(x.to_string())
      } else {
        None
      }
    });
    // println!("{:?}", aft);

//map
    let bef = Some(10);
    let aft = bef.map(|x| x.to_string());
}