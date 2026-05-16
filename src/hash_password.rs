use argon2::{
    Argon2, password_hash::{PasswordHasher, SaltString, rand_core::OsRng}
};

pub fn hash_password(password_string: String) -> String {
    let password = password_string.as_bytes();
    
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password, &salt)
        .unwrap() // Unwrap the Result, panics on error
        .to_string();

    password_hash
}

// pub fn verify_password(password_first: String, password_second: String) -> bool {
//     let argon2 = Argon2::default();

//     let password_first = password_first.as_bytes();

//     let parsed_hash = PasswordHash::new(&password_second).ok().unwrap();

//     argon2.verify_password(password_first, &parsed_hash).is_ok()

// }