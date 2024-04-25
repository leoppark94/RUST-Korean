# RUST-Korean

This library is written in Rust and provides functionalities for processing Korean characters. It supports the identification of Korean characters, composition of Korean syllables from their individual components, classification of Korean characters into specific types, and more. It is designed to support only Modern Korean.

## Features

- `check_korean`: Determines if a given character is a Korean character. This function checks whether a character is within the Unicode range for Korean syllables or is a Korean consonant or vowel.

- `compose_korean`: Composes a complete Korean syllable from separated components such as initial consonant, medial vowel, and final consonant. This feature is ideal for applications requiring the dynamic generation of Korean text.

- `classify_korean`: Classifies a given Korean character into one of the specific types defined in the `HangulType` enum. This function is crucial for applications that need to understand the role of each character in the Korean language.

- `syllable_check`: Returns an `SyllableType` enum indicating the type of Korean syllable component the character is. This function is essential for processing and analyzing Korean text at a granular level.

## SyllableType Enum

The `SyllableType` enum categorizes Korean characters into consonants and vowels, and also identifies numbers and non-Korean characters.
- `FirstConsonantLetter`: Represents initial consonants in a syllable.
- `MiddleVowelLetter`: Represents medial vowels in a syllable.
- `LastConsonantLetter`: Represents final consonants in a syllable.
- `BothFirstLastConsonant`: Represents characters that can function as either initial or final consonants.
- `Number`: Represents numeric characters ('0' through '9').
- `NotConsonant`: Represents characters that are not Korean consonants, vowels, or numbers.

## KoreanType Enum

The `KoreanType` enum is used by the `classify_korean` function to categorize Korean characters into more specific types such as consonants, vowels, complex consonants, and complex vowels, providing a deeper understanding of the character's role in the language.

### Usage

To determine the syllable type of a character, use the `syllable_check` function:

```rust
let character_type = syllable_check('가');
assert_eq!(character_type, SyllableType::NotConsonant);

let number_type = syllable_check('5');
assert_eq!(number_type, SyllableType::Number);
