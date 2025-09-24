use bcrypt::{hash, verify, DEFAULT_COST};
use std::error::Error;

pub fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    Ok(hash(password, DEFAULT_COST)?)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, Box<dyn Error>> {
    Ok(verify(password, hash)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() -> Result<(), Box<dyn Error>> {
        let password = "test_password";
        let hash = hash_password(password)?;
        assert!(verify_password(password, &hash)?);
        assert!(!verify_password("wrong_password", &hash)?);
        Ok(())
    }
}