pub mod enums;
pub mod compose_korean;
pub use compose_korean::compose_korean;

pub use enums::{SyllableType, KoreanType};

/// Checks if a character is a Korean syllable or a complete Korean character.
///
/// # Arguments
/// * `character` - The character to check.
///
/// # Returns
/// * `bool` - Returns `true` if the character is a Korean syllable or a complete Korean character, otherwise `false`.
///
/// # Examples
/// ```
/// use rustkorean::check_korean;
///
/// assert!(check_korean('가'));
/// assert!(!check_korean('A'));
/// ```
pub fn check_korean(character: char) -> bool {
    // check if not a complete combined Korean
    let syllable_check_result = syllable_check(character);
    if syllable_check_result != SyllableType::NotConsonant && syllable_check_result != SyllableType::Number{
       return true
    };
    // Check if the character is within the range of complete Korean characters
    let unicode = character as u32;
    unicode >= 44032 && unicode <= 55203
}

/// Determines the type of Korean syllable for a character.
///
/// # Arguments
/// * `character` - The Korean syllable character to check.
///
/// # Returns
/// * `SyllableType` - The type of Korean syllable.
///
/// # Examples
/// ```
/// use rustkorean::{syllable_check, SyllableType};
///
/// assert_eq!(syllable_check('ㄱ'), SyllableType::BothFirstLastConsonant);
/// assert_eq!(syllable_check('ㅏ'), SyllableType::MiddleVowelLetter);
/// ```
pub fn syllable_check(character: char) -> SyllableType {
    match character {
        'ㄸ' | 'ㅃ' | 'ㅉ' => SyllableType::FirstConsonantLetter,
        'ㅏ' | 'ㅐ' | 'ㅑ' | 'ㅒ' | 'ㅓ' | 'ㅔ' | 'ㅕ' | 'ㅖ' |
        'ㅗ' | 'ㅘ' | 'ㅙ' | 'ㅚ' | 'ㅛ' | 'ㅜ' | 'ㅝ' | 'ㅞ' |
        'ㅟ' | 'ㅠ' | 'ㅡ' | 'ㅢ' | 'ㅣ' => SyllableType::MiddleVowelLetter,
        'ㄳ' | 'ㄵ' | 'ㄶ' | 'ㄺ' | 'ㄻ' | 'ㄼ' | 'ㄽ' | 'ㄾ' |
        'ㄿ' | 'ㅀ' | 'ㅄ' | 'ㅆ' => SyllableType::LastConsonantLetter,
        'ㄱ' | 'ㄲ' | 'ㄴ' | 'ㄷ' | 'ㄹ' | 'ㅁ' | 'ㅂ' | 'ㅅ' |
        'ㅇ' | 'ㅈ' | 'ㅊ' | 'ㅋ' | 'ㅌ' | 'ㅍ' | 'ㅎ' => SyllableType::BothFirstLastConsonant,
        '0'..='9' => SyllableType::Number,
        _ => SyllableType::NotConsonant,
    }
}

/// Checks if the given character is a valid Korean initial consonant (choseong).
///
/// # Arguments
/// * `character` - A `char` representing a single character to check.
///
/// # Returns
/// * `true` if the character is one of the Korean initial consonants.
/// * `false` otherwise.
///
/// # Examples
/// ```
/// use rustkorean::first_letter_check;
///
/// assert!(first_letter_check('ㄱ'));
/// assert!(!first_letter_check('ㅏ'));
/// ```
pub fn first_letter_check(character: char) -> bool {
    match character {
        'ㄱ' | 'ㄲ' | 'ㄴ' | 'ㄷ' | 'ㄸ' | 'ㄹ' | 'ㅁ' | 'ㅂ' |
        'ㅃ' | 'ㅅ' | 'ㅆ' | 'ㅇ' | 'ㅈ' | 'ㅉ' | 'ㅊ' | 'ㅋ' |
        'ㅌ' | 'ㅍ' | 'ㅎ' => true,
        _ => false,
    }
}

/// Checks if the given character is a valid Korean medial vowel (jungseong).
///
/// # Arguments
/// * `character` - A `char` representing a single character to check.
///
/// # Returns
/// * `true` if the character is one of the Korean medial vowels.
/// * `false` otherwise.
///
/// # Examples
/// ```
/// use rustkorean::middle_letter_check;
///
/// assert!(middle_letter_check('ㅏ'));
/// assert!(!middle_letter_check('ㄱ'));
/// ```
pub fn middle_letter_check(character: char) -> bool {
    match character {
        'ㅏ' | 'ㅐ' | 'ㅑ' | 'ㅒ' | 'ㅓ' | 'ㅔ' | 'ㅕ' | 'ㅖ' |
        'ㅗ' | 'ㅘ' | 'ㅙ' | 'ㅚ' | 'ㅛ' | 'ㅜ' | 'ㅝ' | 'ㅞ' |
        'ㅟ' | 'ㅠ' | 'ㅡ' | 'ㅢ' | 'ㅣ' => true,
        _ => false,
    }
}

/// Checks if the given character is a valid Korean final consonant (jongseong).
///
/// # Arguments
/// * `character` - A `char` representing a single character to check.
///
/// # Returns
/// * `true` if the character is one of the Korean final consonants.
/// * `false` otherwise.
///
/// # Examples
/// ```
/// use rustkorean::last_letter_check;
///
/// assert!(last_letter_check('ㅎ'));
/// assert!(!last_letter_check('ㅏ'));
/// ```
pub fn last_letter_check(character: char) -> bool {
    match character {
        'ㄱ' | 'ㄲ' | 'ㄳ' | 'ㄴ' | 'ㄵ' | 'ㄶ' | 'ㄷ' | 'ㄹ' |
        'ㄺ' | 'ㄻ' | 'ㄼ' | 'ㄽ' | 'ㄾ' | 'ㄿ' | 'ㅀ' | 'ㅁ' |
        'ㅂ' | 'ㅄ' | 'ㅅ' | 'ㅆ' | 'ㅇ' | 'ㅈ' | 'ㅊ' | 'ㅋ' |
        'ㅌ' | 'ㅍ' | 'ㅎ' => true,
        _ => false,
    }
}



/// Classifies a given Hangul character into one of the defined Hangul types.
///
/// # Arguments
/// * `character` - A `char` representing a single Hangul character.
///
/// # Returns
/// A `HangulType` indicating the classified type of the Hangul character.
///
/// # Examples
/// ```
/// use rustkorean::{classify_korean, KoreanType};
/// assert_eq!(classify_korean('ㄱ'), KoreanType::Consonant);
/// assert_eq!(classify_korean('ㅏ'), KoreanType::Vowel);
/// assert_eq!(classify_korean('ㄲ'), KoreanType::ComplexConsonant);
/// assert_eq!(classify_korean('ㅐ'), KoreanType::ComplexVowel);
/// assert_eq!(classify_korean('x'), KoreanType::Unknown); // Non-Hangul example
/// ```
pub fn classify_korean(character: char) -> KoreanType {
    match character {
        // Matches basic consonants and maps them to `Consonant`
        'ㄱ' | 'ㄴ' | 'ㄷ' | 'ㄹ' | 'ㅁ' | 'ㅂ' | 'ㅅ' | 'ㅇ' |
        'ㅈ' | 'ㅊ' | 'ㅋ' | 'ㅌ' | 'ㅍ' | 'ㅎ' => KoreanType::Consonant,

        // Matches basic vowels and maps them to `Vowel`
        'ㅏ' | 'ㅑ' | 'ㅓ' | 'ㅕ' | 'ㅗ' | 'ㅛ' | 'ㅜ' | 'ㅠ' |
        'ㅡ' | 'ㅣ' => KoreanType::Vowel,

        // Matches complex or double consonants and maps them to `ComplexConsonant`
        'ㄲ' | 'ㄸ' | 'ㅃ' | 'ㅆ' | 'ㅉ' => KoreanType::ComplexConsonant,

        // Matches complex vowels and maps them to `ComplexVowel`
        'ㅐ' | 'ㅒ' | 'ㅔ' | 'ㅖ' | 'ㅘ' | 'ㅙ' | 'ㅚ' | 'ㅝ' |
        'ㅞ' | 'ㅟ' | 'ㅢ' => KoreanType::ComplexVowel,

        // Matches any other character and maps it to `Unknown`
        _ => KoreanType::Unknown
    }
}

