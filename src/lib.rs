extern crate base64;
extern crate hex;

pub fn hex_to_base64(data: String) -> String {

    let bytes = hex::decode(data).unwrap();
    return base64::encode(&bytes)

}

#[cfg(test)]
mod test {
    use super::hex_to_base64;

    #[test]
    fn convert_hex() {
        let test_input = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        let expected_output = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        assert_eq!(hex_to_base64(test_input), expected_output); 
    }
}
