pub fn print() {
    println!("Hello, world!");
    println!("Number:{}", 1);
    println!("Basic Formating:{} {}","Basic","formatting");
    //positional argument
    println!("{0} is a {1}","Rust","Programming Lang.");

    //named argument
    println!("{language}",language = "Rust");

    //debug trait placeholder for multiple values
    println!("{:?}",(12,"hai",true));

    //arithmetic
    println!("sum of two numbers are:{}", 10 + 10);

    
}