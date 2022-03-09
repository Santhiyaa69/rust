use std::mem;
pub fn arrays() { 

    let mut num = [23,34];
    num[0] = 25;
    println!("{:?}",num);

    let mut numbers: [i32;3] = [23,45,67];
    println!("Array:{:?}",numbers);

    let data: [&str;5] = ["a","b","c","d","e"];

    //get single value
    println!("single element:{}",numbers[2]);

    //re-assign value
    numbers[1] = 44;
    println!("Array:{:?}",numbers);

    //array length
    println!("Array:{:?}",numbers.len());
    
    //slice
    let slice:&[&str] = &data[1..3];
    println!("slice:{:?}",slice);

    //Arrays are stack allocated
    println!("{}",mem::size_of_val(&numbers))

}