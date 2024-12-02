extern crate rustkorean;
use rustkorean::{
    check_korean, classify_korean, first_letter_check, last_letter_check, middle_letter_check,
    syllable_check,
};
fn main() {
    // example of check korean
    let korean_char = '한';
    let non_korean_char = 'A';

    println!(
        "Is '{}' a Korean character? {}",
        korean_char,
        check_korean(korean_char)
    );
    println!(
        "Is '{}' a Korean character? {}",
        non_korean_char,
        check_korean(non_korean_char)
    );

    // example of syllable_check
    let examples = vec!['ㄱ', 'ㅏ', 'ㅣ', '1', 'a', 'ㄳ'];
    for &example in &examples {
        let syllable_type = syllable_check(example);
        println!("{:?} is {:?}", example, syllable_type);
    }

    //example of first_letter_check, middle_letter_check, last_letter_check
    let first_letter = 'ㄱ';
    let middle_letter = 'ㅏ';
    let last_letter = 'ㅎ';
    let not_korean_letter = 'A';

    if first_letter_check(first_letter) {
        println!("'{}' is first letter.", first_letter);
    }

    if middle_letter_check(middle_letter) {
        println!("'{}' is middle letter.", middle_letter);
    }

    if last_letter_check(last_letter) {
        println!("'{}' is last_letter.", last_letter);
    }

    if !first_letter_check(not_korean_letter)
        && !middle_letter_check(not_korean_letter)
        && !last_letter_check(not_korean_letter)
    {
        println!("'{}' is not_korean_letter.", not_korean_letter);
    }

    // example of classify_korean
    let characters = vec!['ㄱ', 'ㅏ', 'ㄲ', 'ㅐ', 'x'];

    for &char in &characters {
        let korean_type = classify_korean(char);
        println!("{:?} is classified as {:?}", char, korean_type);
    }
}
