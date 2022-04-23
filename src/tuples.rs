pub fn tuples() {
//nested tuple
    let num = (1,2,(5,6));
    println!("{:?}",(num.2).1);

    let student:(&str,i32) = ("santhiyaa",23);
    println!("{} {}",student.0,student.1);
}