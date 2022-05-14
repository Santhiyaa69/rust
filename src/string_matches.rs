pub fn run() {
    let pangram: &'static str = "the __INV_1__ brown fox __INV_2__ over the lazy dog";
    println!("{:?}", pangram.chars().count());
    println!("{:?}", pangram.matches("__INV").count());

    let unicode = '\u{20B9}';
    println!("Unicode:{}", unicode);

    // let v =[0xE2, 0x82, 0xB9];
    let v1 = vec![226, 130, 185];
    let v1 = String::from_utf8_lossy(&v1);

    println!("{:?}", v1);
}
