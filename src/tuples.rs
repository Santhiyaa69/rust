pub fn tuples() {
    let num = (1,2);
    println!("{:?}",(num.0,num.1));

    let student:(&str,i32) = ("santhiyaa",23);
    println!("{} {}",student.0,student.1);
}