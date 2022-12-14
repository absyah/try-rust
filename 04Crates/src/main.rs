use std::io;
use our_package::cipher::{rot13, rsa, Cipher};
fn main() {
    println!("Input the string you want to encrypt (ROT13):");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Cannot read input");

    println!(
        "Your encrypted string (ROT13): {}",
        rot13::Rot13(user_input).encrypted_string().unwrap()
    );


    println!("Input the string you want to encrypt (RSA):");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Cannot read input");

    let encrypted_input = rsa::Rsa::new(user_input).expect("");
    let encrypted_string = encrypted_input.encrypted_string().expect("");
    let decrypted_string = encrypted_input.original_string().expect("");

    println!("Your encrypted string (RSA): {}", encrypted_string);
    println!("Your original string (RSA): {}", decrypted_string);
}
