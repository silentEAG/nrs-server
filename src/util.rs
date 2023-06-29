use crate::config::CONFIG;

// TOO SLOW!!!
pub fn calc_password_hash(password: &str, dyn_salt: &str) -> String {
    let argon2 = argon2::Argon2::default();
    let mut output_hash = [0u8; 32];
    argon2
        .hash_password_into(
            password.as_bytes(),
            &[dyn_salt.as_bytes(), CONFIG.server.salt.as_bytes()].concat(),
            &mut output_hash,
        )
        .unwrap();
    hex::encode(output_hash)
}
