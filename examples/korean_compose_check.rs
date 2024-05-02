extern crate rustkorean;
use rustkorean::compose_korean::compose_korean;
fn main() {
    let chars = "ㄴㅏㄴㄷㅙ hi".chars().collect::<Vec<char>>();
    println!("{:?}", chars);
    println!("{}", compose_korean(chars))
}