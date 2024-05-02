extern crate rustkorean;
use rustkorean::{check_korean, middle_letter_check};
fn main() {
    let korean_word1 = 'ㄱ';
    let korean_word2 = '가';
    let korean_word3 = ' ';
    let korean_word4 = '3';
    let korean_word5 = 'a';

    println!("{:?}", check_korean(korean_word1));
    println!("{:?}", check_korean(korean_word2));
    println!("{:?}", check_korean(korean_word3));
    println!("{:?}", check_korean(korean_word4));
    println!("{:?}", check_korean(korean_word5));
}