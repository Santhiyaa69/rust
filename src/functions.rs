//METHOD
//method and functions are not same
//method has self parameter as first 
//invoked by instances   (.)

//ASSOCIATED FUNCTIONS
//doesn't contain self parameter
//invoked by double colon - path separator (::)

//FUNCTIONS
//function's are directly called by it's name

// self is the current module or instance  of the object.
//  &self is a reference the the current object, useful if you want to use the object but not take ownership. 
//  Self refers to the return type of the current object. 

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
 pub fn con (n1: String,n2: String) -> String {
        let join = n1 + &n2;
        join
}
