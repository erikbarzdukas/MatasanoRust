extern crate set1rs;

use set1rs::hex_to_base64;

fn main() {
    println!("{}", hex_to_base64(String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")));;
}
