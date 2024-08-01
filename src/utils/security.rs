// Libs
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};

// Functions
/**
Generates a random salt string.
*/
pub fn generate_salt() -> String {
    SaltString::generate(&mut OsRng).as_str().to_string()
}

/**
Hashes some content using the Argon2 algorithm.
To get a salt, call `generate_salt()`.
*/
pub fn hash_argon2(content: &str, salt: &str) -> String {
    let argon = Argon2::default();
    let mut hash = vec![0u8; 32];
    argon
        .hash_password_into(content.as_bytes(), salt.as_bytes(), &mut hash)
        .unwrap();
    hex::encode(hash)
}
