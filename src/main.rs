extern crate set1rs;

use set1rs::hex_to_base64;
use set1rs::xor_hex;

fn main() {
    solution1();
    solution2();
}

fn solution1() {
    println!("\nSolution 1:\n {}", hex_to_base64(String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")));;
}

fn solution2() {
    let test_input = String::from("1c0111001f010100061a024b53535009181c");
    let test_key = String::from("686974207468652062756c6c277320657965");
    println!("\nSolution 2:\n {}", xor_hex(test_input, test_key));
}
