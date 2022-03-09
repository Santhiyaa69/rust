pub fn run() {
    // let addnum = add(10,7);
    // println!("{}",addnum);

    // let g = greet("hello");
    // println!("{}",g);

    // let data = String::from("Hello");
    // let result =  length(&data);
    // println!("{} {:?}",&data,result);

    let param1 = "hai".to_string();
    let param2 = param1.len();
    let res = multiple_values(&param1,param2);
    println!("{:?}",res);
    println!("{}",param1);

}

// fn add (val1:u8,val2:u8) -> u8 {
//    let result = val1+val2;
//    result
//     //return  val1+val2;
// }

// fn greet (greeting:&str) -> &str {
//     greeting
// }

// fn length(s: &String) -> usize {
//     s.len()
// }

fn multiple_values (p1: &String, p2: usize) ->(&String,usize) {
    (p1,p2)
}