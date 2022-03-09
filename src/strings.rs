pub fn strings() {

    let greet = "hello";
    println!("{}",greet);

    let greet:&str = "hello...";
    println!("{}",greet);

    let empty = String::new();
    println!("{}",empty.len());

    let mut data = String::from("Hello ");
    println!("{}",data);

//push string
    data.push_str("world");

//push char
    data.push('!');

    println!("{}",data);

//length
    println!("length:{}",data.len());
    println!("capacity:{}",data.capacity());

//check if-empty
    println!("Is_Empty:{}",data.is_empty());
    
//check it contains sub-string
    println!("Contains:{}",data.contains("world"));

    assert_eq!(12,data.len());

    for d in data.split_whitespace() {
        println!("{}",d)
    }

}