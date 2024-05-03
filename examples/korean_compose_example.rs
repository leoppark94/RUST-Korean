extern crate rustkorean;
use rustkorean::compose_korean;

fn main() {
    // Convert the sequence of Hangul jamo characters for "안녕하세요" into a Vec<char>.
    let chars = "ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅔㅇㅛ".chars().collect::<Vec<char>>();
    // Call the `compose_korean` function to combine the jamo characters into a complete Hangul string.
    let composed_string = compose_korean(chars);
    // Print the composed string.
    println!("{}", composed_string); // This should print "안녕하세요".
}