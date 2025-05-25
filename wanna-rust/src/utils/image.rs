use std::fs;
use std::io::Error;

pub fn get_img_bytes(path: &str) -> Result<(), Error> {
    let bytes: Vec<u8> = fs::read(path)?;

    print!("\n\nconst IMAGE_BYTES: &[u8] = &[");
    for (i, byte) in bytes.iter().enumerate() {
        if i % 20 == 0 {
            print!("\n    ");
        }
        print!("0x{:02X}, ", byte);
    }

    println!("\n];\n\n");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_img_bytes() {
        let result = get_img_bytes("C:\\picture");
        assert!(result.is_ok());
    }
}
