use std::{
    thread,
    time::Duration,
    process::Command,
    io::{self, Write}
};

use lib::crypto;

fn main() {
    loop {
        print!("identifier: ");
        io::stdout()
            .flush()
            .unwrap();

        let mut identifier: String = String::new();
        io::stdin()
            .read_line(&mut identifier)
            .unwrap();

        print!("master key: ");
        io::stdout()
            .flush()
            .unwrap();

        let mut master_key: String = String::new();
        io::stdin()
            .read_line(&mut master_key)
            .unwrap();

        print!("protected key: ");
        io::stdout()
            .flush()
            .unwrap();

        let mut protected_key: String = String::new();
        io::stdin()
            .read_line(&mut protected_key)
            .unwrap();

        let identifier: &str = identifier.trim();
        let master_key: &str = master_key.trim();
        let protected_key: &str = protected_key.trim();

        println!("\nyou entered:");
        println!("identifier: \"{}\"", identifier);
        println!("master Key: \"{}\"", master_key);
        println!("protected key: \"{}\"", protected_key);
        print!("\nis this correct? (Y/n): ");
        io::stdout()
            .flush()
            .unwrap();

        let mut confirm: String = String::new();
        io::stdin()
            .read_line(&mut confirm)
            .unwrap();

        let confirm: String = confirm
            .trim()
            .to_lowercase();

        if confirm == "n" {
            Command::new("cmd")
                .args(&["/C", "cls"])
                .spawn()
                .expect("faild to run command");

            thread::sleep(Duration::from_millis(40));
        } else {
            handle_input(identifier, master_key, protected_key);
            break;
        }
    }
}

fn handle_input(identifier: &str, master_key: &str, protected_key: &str) {
    let key: String = master_key.to_string() + identifier;
    let actual_key = match crypto::decrypt(&protected_key, &key, true) {
        Err(err) => {
            eprintln!("could not decrypt: {}", err);
            return;
        },

        Ok(res) => res
    };

    println!("decrypted: \"{}\"", actual_key);
}
