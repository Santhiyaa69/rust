pub fn vars() {
let _name = "santhiyaa";
let name = "Santhiyaa";

let mut age = 22;
println!("{} {}",name,age);
println!("{name} {age}",name=_name,age=age);

//re-assign value
age = 23;
println!("{} {}",name,age);

//constant 
const DOB:i32 = 1998;
println!("{}",DOB);

//tuple
println!("{:?}",(name,age,DOB))
}