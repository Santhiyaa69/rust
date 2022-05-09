use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Scrypt,
};

pub fn hash_password(password: &str) {
    let salt = SaltString::generate(&mut OsRng);
    let encrypt = Scrypt.hash_password(password.as_ref(), &salt).unwrap();
    // println!("{}", salt);
    println!("{}", encrypt);
}
