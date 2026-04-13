use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_correct_password() {
        let hash = hash_password("hunter2").expect("hash should succeed");
        assert!(verify_password("hunter2", &hash));
    }

    #[test]
    fn verify_rejects_wrong_password() {
        let hash = hash_password("hunter2").expect("hash should succeed");
        assert!(!verify_password("wrong", &hash));
    }

    #[test]
    fn hash_is_different_each_time() {
        let h1 = hash_password("same").expect("hash should succeed");
        let h2 = hash_password("same").expect("hash should succeed");
        assert_ne!(h1, h2, "argon2 hashes must use unique salts");
    }
}
