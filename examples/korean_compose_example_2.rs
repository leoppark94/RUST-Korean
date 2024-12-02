use rustkorean::{compose_korean, create_double_consonant};

fn main() {
    let input_chars = vec![
        'ㄱ', 'ㅏ', 'ㅂ', 'ㅅ', 'ㅇ', 'ㅡ', 'ㄹ', ' ', 'ㅁ', 'ㅐ', 'ㄱ', 'ㅣ', 'ㄷ', 'ㅏ',
    ];
    let doubled_consonants = create_double_consonant(input_chars);
    let composed_string = compose_korean(doubled_consonants);
    println!("Composed string: {}", composed_string);
}
