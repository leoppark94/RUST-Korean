extern crate rustkorean;
use rustkorean::create_double_consonant;

fn main() {
    let test_cases = vec![
        vec!['ㄱ', 'ㅅ', 'ㄴ', 'ㅈ', 'ㄹ', 'ㅎ'],
        vec!['ㄹ', 'ㄱ', 'ㄹ', 'ㅁ', 'ㄹ', 'ㅂ', 'ㄹ', 'ㅅ'],
        vec!['ㅂ', 'ㅅ'],
    ];

    for case in test_cases {
        let result = create_double_consonant(case);
        println!("Result: {:?}", result.iter().collect::<String>());
    }
}
