use anyhow::Result;
use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const DIGITS: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*_+-=";

/// generate random password
pub fn generate_password(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    digits: bool,
    symbols: bool,
) -> Result<()> {
    let mut password = Vec::new();
    let mut rng = rand::thread_rng();
    let mut chars = Vec::new();
    if uppercase {
        chars.extend_from_slice(UPPER);
        password.push(
            *UPPER
                .choose(&mut rng)
                .expect("UPPER won't be empty in this context"),
        );
    }
    if lowercase {
        chars.extend_from_slice(LOWER);
        password.push(
            *LOWER
                .choose(&mut rng)
                .expect("LOWER won't be empty in this context"),
        );
    }
    if digits {
        chars.extend_from_slice(DIGITS);
        password.push(
            *DIGITS
                .choose(&mut rng)
                .expect("DIGITS won't be empty in this context"),
        );
    }
    if symbols {
        chars.extend_from_slice(SYMBOLS);
        password.push(
            *SYMBOLS
                .choose(&mut rng)
                .expect("SYMBOLS won't be empty in this context"),
        );
    }

    for _ in 0..(length - password.len() as u8) {
        let ch = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*ch);
    }
    password.shuffle(&mut rng);
    println!("{}", String::from_utf8(password)?);
    Ok(())
}
