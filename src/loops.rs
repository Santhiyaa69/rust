pub fn loops() {

//for loop
    let mut v:Vec<i32> = vec![];
    for value in 1..50 {
        if value%5 == 0 {
            v.push(value);
        }
    }
    println!("{:?}",v);

//while loop
    // let mut count = 0;
    // while count <= 25{
    //     if count%3 == 0{
    //         println!("Fizz");
    //     } else if count%5 == 0{
    //         println!("buzz");
    //     } else {
    //         println!("{}",count);
    //     }
    //     count+=1;
    // }

//loop

    let mut x = 0;
    loop {
        x += 1;
        println!("{}",x);
        if x == 5 {
            break;
        }
    }
}