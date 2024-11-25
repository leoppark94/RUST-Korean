# RustKorean
RustKorean is a Rust library for processing Korean characters. It provides functionalities to check if a character is Korean, classify Korean characters, verify if a character is a leading consonant (choseong), a medial vowel (jungseong), or a trailing consonant (jongseong) in the Korean writing system, combine Jamo characters into complete Hangul syllables, and check the syllable type of characters.
***

## Features

- Check if a character is Korean (`check_korean`)
- Classify Korean characters (`classify_korean`)
- Check for leading consonants (Choseong) (`first_letter_check`)
- Check for medial vowels (Jungseong) (`middle_letter_check`)
- Check for trailing consonants (Jongseong) (`last_letter_check`)
- Combine Jamo characters into Hangul (`compose_korean`)
- Check the syllable type of characters (`syllable_check`)
- Combine individual Korean Jamo characters into double consonants where applicable (`create_double_consonant`)
- Convert English input to korean (`english_input_to_korean`)
***

## Usage

First, add `rustkorean` as a dependency in your `Cargo.toml` file.

```toml
[dependencies]
rustkorean = "1.0.2"
```
***

### Checking if a Character is Korean

Example to check if a character is Korean.

```rust
extern crate rustkorean;
use rustkorean::check_korean;

fn main() {
    let korean_char = '한';
    let non_korean_char = 'A';

    println!("Is '{}' a Korean character? {}", korean_char, check_korean(korean_char));
    println!("Is '{}' a Korean character? {}", non_korean_char, check_korean(non_korean_char));
}
```
***
### Checking the Syllable Type

Example to check the syllable type of characters.

```rust
extern crate rustkorean;
use rustkorean::syllable_check;

fn main() {
    let examples = vec!['ㄱ', 'ㅏ', 'ㅣ', '1', 'a', 'ㄳ'];
    for &example in &examples {
        let syllable_type = syllable_check(example);
        println!("{:?} is {:?}", example, syllable_type);
    }
}
```
***
### Checking for Choseong, Jungseong, and Jongseong

Example to verify if a character is a leading consonant, a medial vowel, or a trailing consonant.

```rust
extern crate rustkorean;
use rustkorean::{first_letter_check, middle_letter_check, last_letter_check};

fn main() {
    let first_letter = 'ㄱ';
    let middle_letter = 'ㅏ';
    let last_letter = 'ㅎ';
    let not_korean_letter = 'A';

    if first_letter_check(first_letter) {
        println!("'{}' is a first letter.", first_letter);
    }

    if middle_letter_check(middle_letter) {
        println!("'{}' is a middle letter.", middle_letter);
    }

    if last_letter_check(last_letter) {
        println!("'{}' is a last letter.", last_letter);
    }

    if !first_letter_check(not_korean_letter) && !middle_letter_check(not_korean_letter) && !last_letter_check(not_korean_letter) {
        println!("'{}' is not a Korean letter.", not_korean_letter);
    }
}
```
***
### Combining Jamo Characters into Hangul

Example to combine Jamo characters into complete Hangul syllables.

```rust
extern crate rustkorean;
use rustkorean::compose_korean;

fn main() {
    let chars = "ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅔㅇㅛ".chars().collect::<Vec<char>>();
    let composed_string = compose_korean(chars);
    println!("{}", composed_string); // This should print "안녕하세요".
}
```
***

### Combining Separated Double Consonants into Complete Korean Characters

This functionality combines input with separated double consonants into complete Hangul characters.

It works well in conjunction with the `compose_korean` function.

Please refer to `korean_compose_example_2.rs` for an example.

```rust
extern crate rustkorean;
use rustkorean::create_double_consonant;

fn main() {
    let test_cases = vec![
        vec!['ㄱ', 'ㅅ', 'ㄴ', 'ㅈ', 'ㄹ', 'ㅎ'],
        vec!['ㄹ', 'ㄱ', 'ㄹ', 'ㅁ', 'ㄹ', 'ㅂ', 'ㄹ', 'ㅅ']
    ];

    for case in test_cases {
        let result = create_double_consonant(case);
        println!("Result: {:?}", result.iter().collect::<String>());
        // This code will output
        // Result: "ㄳㄵㅀ"
        // Result: "ㄺㄻㄼㄽ"
    }
}
```
***

This section of the README file describes how to process input containing separated double consonants and combine them into complete Hangul syllables, enhancing the functionality of the `compose_korean` function.

## Contributing

If you would like to contribute to this project, pull requests are welcome. If you find any bugs or have a feature request, please open an issue.

***
# 한국어 설명 RustKorean

RustKorean은 한글 문자 처리를 위한 Rust 라이브러리입니다. 한글 문자가 맞는지 확인, 한글 문자 분류, 초성/중성/종성 확인, 자모 문자를 조합하여 완성형 한글 만들기, 문자의 음절 유형 확인 등의 기능을 제공합니다.

## 기능

- 한글 문자 확인 (`check_korean`)
- 한글 문자 분류 (`classify_korean`)
- 초성 확인 (`first_letter_check`)
- 중성 확인 (`middle_letter_check`)
- 종성 확인 (`last_letter_check`)
- 자모 문자를 조합하여 한글 만들기 (`compose_korean`)
- 문자의 음절 유형 확인 (`syllable_check`)
- 한글 자모 문자들을 입력받아 가능한 겹자음을 조합 (`create_double_consonant`)

## 사용 방법

먼저, `Cargo.toml` 파일에 `rustkorean`을 의존성으로 추가합니다.

```toml
[dependencies]
rustkorean = "1.0.2"
```

### 한글 문자 확인

한글 문자인지 확인하는 예시입니다.

```rust
extern crate rustkorean;
use rustkorean::check_korean;

fn main() {
    let korean_char = '한';
    let non_korean_char = 'A';

    println!("'{}'는 한글 문자인가? {}", korean_char, check_korean(korean_char));
    println!("'{}'는 한글 문자인가? {}", non_korean_char, check_korean(non_korean_char));
}
```

### 문자의 음절 유형 확인

문자의 음절 유형을 확인하는 예시입니다.

```rust
extern crate rustkorean;
use rustkorean::syllable_check;

fn main() {
    let examples = vec!['ㄱ', 'ㅏ', 'ㅣ', '1', 'a', 'ㄳ'];
    for &example in &examples {
        let syllable_type = syllable_check(example);
        println!("{:?}는 {:?} 유형입니다.", example, syllable_type);
    }
}
```

### 초성/중성/종성 확인

초성, 중성, 종성인지 확인하는 예시입니다.

```rust
extern crate rustkorean;
use rustkorean::{first_letter_check, middle_letter_check, last_letter_check};

fn main() {
    let first_letter = 'ㄱ';
    let middle_letter = 'ㅏ';
    let last_letter = 'ㅎ';
    let not_korean_letter = 'A';

    if first_letter_check(first_letter) {
        println!("'{}'는 초성입니다.", first_letter);
    }

    if middle_letter_check(middle_letter) {
        println!("'{}'는 중성입니다.", middle_letter);
    }

    if last_letter_check(last_letter) {
        println!("'{}'는 종성입니다.", last_letter);
    }

    if !first_letter_check(not_korean_letter) && !middle_letter_check(not_korean_letter) && !last_letter_check(not_korean_letter) {
        println!("'{}'는 한글 문자가 아닙니다.", not_korean_letter);
    }
}
```

### 자모 문자 조합하여 한글 만들기

자모 문자를 조합하여 완성형 한글을 만드는 예시입니다.

```rust
extern crate rustkorean;
use rustkorean::compose_korean;

fn main() {
    let chars = "ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅔㅇㅛ".chars().collect::<Vec<char>>();
    let composed_string = compose_korean(chars);
    println!("{}", composed_string); // 이 코드는 "안녕하세요"를 출력합니다.
}
```
***

### 겹자음이 분리된 문자 조합하여 한글 만들기

겹자음이 분리된 상태의 입력을 조합하여 완성형 한글을 만들어 냅니다.

`compose_korean` 함수와 함께 사용하면 좋습니다.

`korean_compose_example_2.rs`를 참조해주세요.

```rust
extern crate rustkorean;
use rustkorean::create_double_consonant;

fn main() {
    let test_cases = vec![
        vec!['ㄱ', 'ㅅ', 'ㄴ', 'ㅈ', 'ㄹ', 'ㅎ'],
        vec!['ㄹ', 'ㄱ', 'ㄹ', 'ㅁ', 'ㄹ', 'ㅂ', 'ㄹ', 'ㅅ']
    ];

    for case in test_cases {
        let result = create_double_consonant(case);
        println!("Result: {:?}", result.iter().collect::<String>());
        // 이 코드는
        // Result: "ㄳㄵㅀ"
        // Result: "ㄺㄻㄼㄽ"
        // 을 출력합니다

    }
}
```

## 기여하기

이 프로젝트에 기여하고 싶으시다면, 풀 리퀘스트를 환영합니다. 버그를 발견하셨거나 기능 요청이 있으시면, 이슈를 등록해 주세요.


