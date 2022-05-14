//String to &str  - as_str(); - static string
//&str to String  - to_string(); - dynamic string

pub fn strings() {
    let greet = "hello";
    // println!("{}",greet);

    let greet: &str = "hello...";
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
    let char: i32 = char.trim().parse().unwrap();
    // println!("{}", char );

    let a = "5";
    let b = a;
    // println!("{}", b);
    // println!("{}", a);

    //Extract last 3 characters from string in Rust

    let s = String::from("Japan");
    println!("{}", s[s.len() - 3..].to_string()); //pan

    let s = "Santhiyaa";
    println!("{}", s[s.len() - 6..].to_string()); //thiyaa

    //Extract first 5 characters from string in Rust
    let s = String::from("Japanese to English");
    let sub = &s[..5];
    println!("{}", sub); //Japan

    let s = "Japanese to English".chars();
    let sub: String = s.into_iter().take(5).collect();
    println!("{}", sub); //Japan

    //Remove first character from string
    let mut a: String = "SANTHIYAA".to_string();
    a.remove(0);
    println!("{}", a); // ANTHIYAA

    //Remove last character from string
    let mut a: String = "SANTHIYAA".to_string();
    a.pop();
    println!("{}", a); // SANTHIYA

    //Remove specific character from string
    let s: String = "Hello, World!"
        .chars()
        .map(|x| match x {
            '!' => '?',
            'A'..='Z' => 'X',
            'a'..='z' => 'x',
            _ => x,
        })
        .collect();
    println!("{}", s);

    //Add & Remove character from string
    let mut a: String = "Rat".to_string();
    println!("{}", a); // Rat

    a.remove(0);
    println!("{}", a); // at

    a.insert(0, 'C');
    println!("{}", a); // Cat

    a.pop();
    println!("{}", a); // Ca

    a.push('r');
    println!("{}", a); // Car

    a.insert(3, 's');
    println!("{}", a); // Cars
}
