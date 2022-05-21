use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};

fn main() {
    let password = b"hunter42"; // Bad password; don't actually use!
    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string ($pbkdf2-sha256$...)
    let password_hash_result = Pbkdf2.hash_password(password, &salt);

    match password_hash_result {
        Ok(password_hash) => {
            let password_hash_string = password_hash.to_string();
            let parsed_hash_result = PasswordHash::new(&password_hash_string);
            match parsed_hash_result {
                Ok(parsed_hash) => {
                    assert!(Pbkdf2.verify_password(password, &parsed_hash).is_ok());
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
    // Verify password against PHC string
    //let parsed_hash = PasswordHash::new(&password_hash)?;
    //assert!(Pbkdf2.verify_password(password, &parsed_hash).is_ok());
}
