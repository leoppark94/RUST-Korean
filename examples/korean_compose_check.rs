extern crate rustkorean;
use rustkorean::compose_korean::compose_korean;
fn main() {
    let chars = "ㅇㅏㄴㄷㅙ hi".chars().collect::<Vec<char>>();
    println!("{}", compose_korean(chars))
}