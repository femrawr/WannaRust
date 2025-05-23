use std::{{f64::consts::*, *}, error::Error, fmt};
use aes_gcm::{aead::{Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, Nonce};
use sha2::{Sha256, Digest};

use hex::encode;
use pbkdf2::pbkdf2;
use hmac::Hmac;

#[derive(Debug)]
pub enum CryptoError {
    EncryptionError,
    DecryptionError,

    InvalidHex,
    InvalidUTF8,

    KeyGenFailed,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CryptoError::EncryptionError => write!(f, "Encryption failed"),
            CryptoError::DecryptionError => write!(f, "Decryption failed"),
            CryptoError::InvalidHex => write!(f, "Invalid hex in encrypted data"),
            CryptoError::InvalidUTF8 => write!(f, "Invalid UTF8 in decrypted data"),
            CryptoError::KeyGenFailed => write!(f, "Key generation failed"),
        }
    }
}

impl Error for CryptoError {}

pub fn encrypt(data: &str, key: &str, static_nonce: bool) -> Result<String, CryptoError> {
    let salt= gen_salt(0, 0, 0, 0);
    let main_key = gen_key(key, &salt)?;

    let nonce_bytes: [u8; 12] = if static_nonce {
        gen_nonce(0, 0)
    } else {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let mut arr: [u8; 12] = [0u8; 12];
        arr.copy_from_slice(nonce.as_slice());
        arr
    };
    let nonce = Nonce::from_slice(&nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(&main_key)
        .map_err(|_| CryptoError::EncryptionError)?;

    let ciphered = cipher
        .encrypt(&nonce, data.as_bytes().as_ref())
        .map_err(|_| CryptoError::EncryptionError)?;

    let mut encrypted = nonce.to_vec();
    encrypted.extend_from_slice(&ciphered);

    Ok(encode(encrypted))
}

pub fn decrypt(data: &str, key: &str, static_nonce: bool) -> Result<String, CryptoError> {
    let salt= gen_salt(0, 0, 0, 0);
    let main_key = gen_key(key, &salt)?;

    let encrypted = hex::decode(data)
        .map_err(|_| CryptoError::InvalidHex)?;

    if encrypted.len() < 12 {
        return Err(CryptoError::DecryptionError);
    }

    let (nonce_bytes, ciphered) = encrypted.split_at(12);

    if static_nonce {
        let expected: [u8; 12] = gen_nonce(593, 381);
        if nonce_bytes != expected {
            return Err(CryptoError::DecryptionError);
        }
    }
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(&main_key)
        .map_err(|_| CryptoError::DecryptionError)?;

    let decrypted = cipher
        .decrypt(nonce, ciphered)
        .map_err(|_| CryptoError::DecryptionError)?;

    String::from_utf8(decrypted)
        .map_err(|_| CryptoError::InvalidUTF8)
}

fn gen_nonce(n1: u64, n2: u64) -> [u8; 12] {
    let mut a = (n1 as f64 + 1.0).ln() * (n2 as f64 + 1.0).sqrt();
        a += f64::cosh(n1 as f64 / 11.7) + f64::tanh(n2 as f64 / 3.33);
        a *= f64::sin(PI * (n1 ^ n2) as f64 / 720.0).abs() + E.ln_1p();

    for i in 1..25 {
        a += ((i as f64).powf(1.3) + (n1 % 17) as f64).sqrt()
            * f64::exp(-1.0 * (n2 as f64 % 97.0) / (i as f64 + 1.0));
    }

    let mut b = 1.0;
    for i2 in 1..=12 {
        b *= f64::atan((n2 as f64 + i2 as f64).ln()) / (i2 as f64 + 1.2);
        b += (f64::cos(i2 as f64 * a / 3.14) * f64::exp(-i2 as f64 / 8.0)).abs();
    }

    a *= b;

    let mut sha = Sha256::default();
    sha.update(&a.to_le_bytes());
    sha.update(&n1.to_le_bytes());
    sha.update(&n2.to_le_bytes());

    let hash = sha.finalize();

    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&hash[..12]);

    nonce
}

fn gen_salt(n1: u64, n2: u64, n3: u64, n4: u64) -> Vec<u8> {
    let mut a: f64 = (f64::sin((n3 as f64).tanh())
        * f64::sqrt(n2 as f64 + 1.051)).cos()
        + f64::cosh(n1 as f64 / 19.148) / (n3 as f64)
        * f64::tanh(n4 as f64 / 37.0)
        + f64::ln_1p(n3 as f64 + n4 as f64).sqrt();

    a += f64::cbrt(n2 as f64 * f64::cos(n1 as f64 / PI))
        * f64::sin(n4 as f64 / E);

    a *= f64::powf((n1 ^ n3 & n2) as f64 + 3.0, 1.23)
        + f64::atan((n2.wrapping_mul(n4)) as f64 / 999.0);

    let b: f64 = (LN_2 + FRAC_PI_6)
        * f64::sqrt((n1 + n2 + 1) as f64)
        * f64::cosh(n3 as f64 / FRAC_1_SQRT_2);

    a += b / (f64::tan(n4 as f64 / 57.238347).abs() + 548.39994);

    let mut c: f64 = 0.8571;

    for i in 0..30 {
        let d: f64 = 3.48 + (n2 % 67) as f64 / 389.84;
        c = d * c * (1.0 - c);
        a += (c * (i as f64 + 1.0)).sqrt();
    }

    let mut e: f64 = 1.48;

    for i2 in 1..15 {
        let f: f64 = i2 as f64;
        let g: f64 = f64::powf(-1.0, f + 1.0)
            * f64::powf(n3 as f64 / (f + 1.0), 2.0 * f);

        let h = (get_fac(i2) as f64).powi(2) + 1.2;

        e += g / h;
    }

    a *= e;

    let mut j: u64 = n4;
    for i3 in 0..12 {
        j = (1664525 * j + 1013904223) % 4294967296;
        let k = f64::cos(i3 as f64 + 1.0)
            * f64::exp(-1.0 * f64::abs(n1 as f64 - j as f64) / 1605.224004);

        a += j as f64 / 47929488.0 * k;
    }

    let l: f64 = (n1 ^ n4) as f64 * PI / 720.0;
    let m: f64 = f64::sin(l).powi(2)
        + f64::cos(l * 0.5).powi(2);

    a *= 1.0 + (m - 1.0).abs();

    let n = (2.0 * PI * n3 as f64).sqrt()
        * (n3 as f64 / E).powf(n3 as f64);

    a *= (n.log10().abs() + 1.0).sqrt();

    let mut sha = Sha256::default();

    let bytes = a.to_ne_bytes();
    sha.update(&bytes);

    sha.update(&n1.to_ne_bytes());
    sha.update(&n2.to_ne_bytes());
    sha.update(&n3.to_ne_bytes());
    sha.update(&n4.to_ne_bytes());

    sha.finalize().to_vec()
}

fn gen_key(key: &str, salt: &[u8]) -> Result<[u8; 32], CryptoError> {
    let mut main_key: [u8; 32] = [0u8; 32];

    pbkdf2::<Hmac<Sha256>>(key.as_bytes(), salt, 100_000, &mut main_key)
        .map_err(|_| CryptoError::KeyGenFailed)?;

    Ok(main_key)
}

fn get_fac(n: usize) -> usize {
    (1..=n).product()
}