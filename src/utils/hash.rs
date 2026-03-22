use argon2::password_hash::{Error, SaltString, rand_core::OsRng};
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};

pub fn hash_password(password: &[u8]) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);

    let m_cost = 1024 * 16; // 16MB
    let t_cost = 3;
    let p_cost = 1;
    let params = Params::new(m_cost, t_cost, p_cost, None)?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let password_hash = argon2.hash_password(password, &salt)?.to_string();

    Ok(password_hash)
}
