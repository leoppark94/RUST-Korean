extern crate rustkorean;
use rustkorean::check_korean;
fn main() {
    let korean_word1 = 'ㄱ';
    let korean_word2 = '가';
    println!("{:?}", check_korean(korean_word1));
    println!("{:?}", check_korean(korean_word2));
}