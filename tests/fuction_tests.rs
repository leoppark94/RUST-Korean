extern crate rustkorean;
use rustkorean::{check_korean, classify_korean, compose_korean, first_letter_check, KoreanType, last_letter_check, middle_letter_check, syllable_check, SyllableType};
use rustkorean::compose_korean::{combine_status_check, make_one_letter};

#[test]
fn test_check_korean() {
    assert!(check_korean('한'));
    assert!(!check_korean('A'));
    assert!(check_korean('힣'));
    assert!(!check_korean('1'));
}

#[test]
fn test_syllable_check() {
    assert_eq!(syllable_check('ㄸ'), SyllableType::FirstConsonantLetter);
    assert_eq!(syllable_check('ㅏ'), SyllableType::MiddleVowelLetter);
    assert_eq!(syllable_check('ㅆ'), SyllableType::LastConsonantLetter);
    assert_eq!(syllable_check('ㅇ'), SyllableType::BothFirstLastConsonant);
    assert_eq!(syllable_check('3'), SyllableType::Number);
    assert_eq!(syllable_check('A'), SyllableType::NotConsonant);
}

#[test]
fn test_first_letter_check() {
    assert!(first_letter_check('ㄱ'));
    assert!(first_letter_check('ㄴ'));
    assert!(!first_letter_check('ㅏ'));
    assert!(!first_letter_check('A'));
    assert!(!first_letter_check('1'));
}

#[test]
fn test_middle_letter_check() {
    assert!(middle_letter_check('ㅏ'));
    assert!(middle_letter_check('ㅓ'));
    assert!(!middle_letter_check('3'));
    assert!(!middle_letter_check('ㄱ'));
    assert!(!middle_letter_check('ㅆ'));
}

#[test]
fn test_last_letter_check() {
    assert!(last_letter_check('ㅎ'));
    assert!(last_letter_check('ㄲ'));
    assert!(last_letter_check('ㄺ'));
    assert!(!last_letter_check('ㅏ'));
    assert!(!last_letter_check('3'));
    assert!(!last_letter_check('A'));
}

#[test]
fn test_classify_korean () {
    assert_eq!(classify_korean('ㄱ'), KoreanType::Consonant);
    assert_eq!(classify_korean('ㅏ'), KoreanType::Vowel);
    assert_eq!(classify_korean('ㄲ'), KoreanType::ComplexConsonant);
    assert_eq!(classify_korean('ㅐ'), KoreanType::ComplexVowel);
    assert_eq!(classify_korean('x'), KoreanType::Unknown);
}

#[test]
fn test_compose_korean() {
    let chars_vec = vec!['ㅎ', 'ㅏ', 'ㄴ', 'ㄱ', 'ㅡ', 'ㄹ',' ', 'A', 'B', 'C', '1', '2', '3', '.'];
    let result = compose_korean(chars_vec);
    assert_eq!(result, "한글 ABC123.");
}

#[test]
fn test_combine_status_check() {
    assert!(combine_status_check(&"ㄱ".to_string(), &'ㅏ'));
    assert!(!combine_status_check(&"가".to_string(), &'ㄱ'));
}

#[test]
fn test_make_one_letter() {
    assert_eq!(make_one_letter("가".to_string()), '가');
    assert_eq!(make_one_letter("ㄱㅏ".to_string()), '가');
}