extern crate base64;
extern crate hex;

use std::collections::HashMap;

pub fn hex_to_base64(data: String) -> String {

    let bytes = hex::decode(data).unwrap();
    return base64::encode(&bytes)

}

pub fn xor_hex(ciphertext: String, key: String) -> String {

    let cipher_bytes = hex::decode(ciphertext).unwrap();
    let key_bytes =  hex::decode(key).unwrap();

    let mut result_bytes = Vec::new();

    for i in 0..cipher_bytes.len() {
        result_bytes.push(cipher_bytes[i] ^ key_bytes[i]);
    }

    return hex::encode(result_bytes)

}

pub fn character_frequency(data: &String) -> HashMap<char, i32> {

    let mut char_map: HashMap<char, i32> = HashMap::new();

    for i in data.to_uppercase().chars().collect::<Vec<char>>() {
        *char_map.entry(i).or_insert(0) += 1;
    }

    return char_map;

}

pub fn sort_char_map(char_map: HashMap<char, i32>) -> String {
    unimplemented!()

}

pub fn score_etaoin(data: String) -> i32 {

    let etaoin = "ETAOIN";
    let vkjxqz = "VKJXQZ";
    let mut score = 0;

    //Invalid string length to operate on, return score of 0
    //Insert better error handling after prototype
    if data.len() <= 11 {
        return score;
    }

    //Score first 6 chars of string against ETAOIN
    //and last 6 against VKJXQZ
    let first_six = &data[0..6];
    let last_six = &data[data.len()-6..data.len()];

    for i in first_six.chars() {
        if etaoin.contains(i) {
            score +=1;
        }
    }

    for i in last_six.chars() {
        if vkjxqz.contains(i) {
            score += 1;
        }
    }

    score
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::hex_to_base64;
    use super::xor_hex;
    use super::character_frequency;
    use super::sort_char_map;
    use super::score_etaoin;

    #[test]
    fn convert_hex() {
        let test_input = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        let expected_output = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        assert_eq!(hex_to_base64(test_input), expected_output); 
    }

    #[test]
    fn test_xor() {
        let test_input = String::from("1c0111001f010100061a024b53535009181c");
        let test_key = String::from("686974207468652062756c6c277320657965");
        let expected_result = String::from("746865206b696420646f6e277420706c6179");
        assert_eq!(xor_hex(test_input, test_key), expected_result);
    }

    #[test]
    fn test_character_freq() {
        let test_string = String::from("eiIeio");
        let mut answer_map: HashMap<char, i32> = HashMap::new();
        answer_map.insert('E', 2);
        answer_map.insert('I', 3);
        answer_map.insert('O', 1);

        assert_eq!(character_frequency(&test_string), answer_map);
    }

    #[test]
    fn test_character_ordering() {
        let mut test_map: HashMap<char, i32> = HashMap::new();
        test_map.insert('E', 2);
        test_map.insert('I', 3);
        test_map.insert('O', 1);

        assert_eq!(sort_char_map(test_map), String::from("IEO"));
    }

    #[test]
    fn test_score_etaoin() {
        assert_eq!(score_etaoin(String::from("ETAOINVKJXQZ")), 12);
    }

    #[test]
    fn test_score_etaoin_short_string() {
        assert_eq!(score_etaoin(String::from("ETAOIN")), 0);
    }
}
