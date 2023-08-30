use std::sync::Mutex;

pub fn mutex() {
    let mut mutex_var = Mutex::new(2);
    println!("mutex_var1 = {:?}", &mutex_var); //2
    let mut mutex_changer = mutex_var.lock().unwrap();
    println!("mutex_var2 = {:?}", &mutex_var); //locked
    println!("mutex_changer1 = {:?}", &mutex_changer); //2
    *mutex_changer = 5;
    println!("mutex_changer2 = {:?}", &mutex_changer); //5
    std::mem::drop(mutex_changer); //unlocked
    mutex_var = Mutex::new(8);
    println!("mutex_var3 = {:?}", &mutex_var); //8
}
