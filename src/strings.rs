pub fn strings() {

    let greet = "hello";
    // println!("{}",greet);

    let greet:&str = "hello...";
    // println!("{}",greet);

    let empty = String::new();
    // println!("{}",empty.len());

    let mut data = String::from("Hello ");
    // println!("{}",data);

//push string
    // data.push_str("world");

//push char
    data.push('!');

    // println!("{}",data);

//length
    // println!("length:{}",data.len());
    // println!("capacity:{}",data.capacity());

//check if-empty
    // println!("Is_Empty:{}",data.is_empty());
    
//check it contains sub-string
    // println!("Contains:{}",data.contains("world"));

   

    for d in data.split_whitespace() {
        // println!("{}",d)
    }

   let s = 27;
//    println!("{:?}", s.to_string());
  
  let n = "54".to_string();
//   println!("{:?}",n.parse::<i32>().unwrap());

//   let my_num: i32 = n.trim().parse()
//    .expect("please give me correct string number!");
//    println!("{}",my_num);

    let char = "50";
    let char : i32 = char.trim().parse().unwrap();
    // println!("{}", char );

    let a = "5";
    let b = a;
    // println!("{}", b);
    // println!("{}", a);
}