pub fn hash(password: &str, salt: &str) -> [u8; 64] {
    let digest = crypto::sha2::Sha512::new();
    let mut mac = crypto::hmac::Hmac::new(digest, &password.as_bytes());
    let mut buf = [0u8; 64];
    crypto::pbkdf2::pbkdf2(&mut mac, &salt.as_bytes(), 1000, &mut buf);
    buf
}
