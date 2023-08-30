use std::any::TypeId;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn run() {
    let string = "Thiya".to_string();
     print_type_of(&string);
    let no = 10;
    print_type_of(&no);
    println!("type-id = {:?}", TypeId::of::<i32>())

}