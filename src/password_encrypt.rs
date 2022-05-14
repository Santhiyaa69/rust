use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};

pub fn hash_password(password: &str) {
    let salt = SaltString::generate(&mut OsRng);
    let encrypt = Scrypt.hash_password(password.as_ref(), &salt).unwrap();
    // println!("{}", salt);
    println!("{}", encrypt);
}

pub fn verify_password(hash: &str, password: &str) {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    let verified = Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    println!("{}", verified);
}
