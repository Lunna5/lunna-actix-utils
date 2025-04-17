use argon2::password_hash::SaltString;
use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use std::sync::{Arc, Mutex};

pub struct HashService {
    salt: Arc<Mutex<SaltString>>,
    argon2: Argon2<'static>,
}

impl HashService {
    pub fn new(salt: &str) -> HashService {
        let salt = SaltString::encode_b64(salt.as_bytes()).expect("salt is invalid");

        HashService {
            salt: Arc::new(Mutex::new(salt)),
            argon2: Argon2::default(),
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<String, password_hash::Error> {
        let salt = self.salt.lock().unwrap();

        match self.argon2.hash_password(password.as_bytes(), &*salt) {
            Ok(result) => Ok(result.to_string()),
            Err(err) => Err(err),
        }
    }

    pub fn verify_password(
        &self,
        password: &str,
        hashed_password: &str,
    ) -> Result<bool, password_hash::Error> {
        let parsed_hash = PasswordHash::new(hashed_password)?;

        let result = self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SALT: &str = "z))IEw6Tph?7(TY83[`2";

    #[test]
    fn test_hash_and_verify_success() {
        let service = get_hash_service();
        let password = "password123";
        let hash = service.hash_password(password).unwrap();
        assert!(service.verify_password(password, &hash).unwrap());
    }

    #[test]
    fn test_hash_and_verify_failure() {
        let service = get_hash_service();
        let password = "password123";
        let hash = service.hash_password(password).unwrap();
        assert!(!service.verify_password("wrongpassword", &hash).unwrap());
    }

    #[test]
    fn test_empty_password() {
        let service = get_hash_service();
        let password = "";
        let hash = service.hash_password(password).unwrap();
        assert!(service.verify_password(password, &hash).unwrap());
    }

    #[test]
    fn test_consistent_hashing() {
        let service = get_hash_service();
        let password = "consistent_password";
        let hash1 = service.hash_password(password).unwrap();
        let hash2 = service.hash_password(password).unwrap();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_verify_invalid_hash_returns_error() {
        let service = get_hash_service();
        let result = service.verify_password("password", "invalid_hash");
        assert!(result.is_err());
    }

    fn get_hash_service() -> HashService {
        HashService::new(SALT)
    }
}