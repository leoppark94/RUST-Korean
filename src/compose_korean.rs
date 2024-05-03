use crate::{check_korean, first_letter_check, last_letter_check, middle_letter_check};

/// Composes a Korean string from a vector of individual Hangul characters (jamo),
/// combining them into complete syllables where possible.
///
/// This function processes the given Hangul jamo characters (`chars_vec`) sequentially,
/// combining them into complete Hangul syllables according to the rules for forming
/// Korean syllables. The jamo are combined in the order of initial consonant (choseong),
/// medial vowel (jungseong), and final consonant (jongseong).
///
/// # Arguments
///
/// * `chars_vec` - A `Vec<char>` vector containing Hangul jamo characters.
///
/// # Returns
///
/// * `String` - The composed string of complete Hangul syllables.
///
/// # Examples
///
/// Basic usage example:
///
/// ```
/// use rustkorean::compose_korean;
/// let chars_vec = vec!['ㅎ', 'ㅏ', 'ㄴ', 'ㄱ', 'ㅡ', 'ㄹ'];
/// let result = compose_korean(chars_vec);
/// assert_eq!(result, "한글");
/// ```
///
/// This function uses the `combine_status_check` function to determine whether each jamo
/// can be added to the currently forming syllable string, and the `make_one_letter` function
/// to actually combine the jamo into complete Hangul syllables. If the jamo characters cannot
/// be combined according to the rules, each jamo is added as an individual character to the
/// resulting string.
pub fn compose_korean(chars_vec: Vec<char>) -> String {
    // where to save the final result
    let mut result = String::new();
    // making the temp one korean char
    let mut combined_one_char = String::new();

    for one_char in chars_vec {
        // maximum length check
        if combined_one_char.chars().count() == 3 {
            // last_letter could combine with next middle_letter
            if middle_letter_check(one_char) {
                let split_last_letter = combined_one_char.pop().unwrap();
                result.push(make_one_letter(combined_one_char.clone()));
                combined_one_char.clear();
                combined_one_char.push(split_last_letter);

            // else just push and combine as a result
            } else {
                result.push(make_one_letter(combined_one_char.clone()));
                combined_one_char.clear();
            }
        }

        // combined_logic started
        if combine_status_check(&combined_one_char, &one_char) {
            combined_one_char.push(one_char);
        } else {
            // if exist previous saving temp
            if !combined_one_char.is_empty() {
                result.push(make_one_letter(combined_one_char.clone()));
                combined_one_char.clear();
                result.push(one_char);
            } else {
                result.push(one_char);
            }
        }
    }

    if !combined_one_char.is_empty() {
        result.push(make_one_letter(combined_one_char.clone()));
    }

    result
}

/// Checks if a character can be combined with a given string to form a valid Korean syllable.
///
/// # Arguments
/// * `combined_one_char` - A reference to a `String` representing the current combined syllable.
/// * `one_char` - A reference to a `char` to be combined.
///
/// # Returns
/// * `bool` - Returns `true` if the character can be combined with the given string to form a valid Korean syllable, otherwise `false`.
///
/// # Panics
/// * The function panics if the `combined_one_char` contains more than 2 characters.
///
/// # Examples
/// ```
/// use rustkorean::compose_korean::combine_status_check;
/// assert!(combine_status_check(&"ㄱ".to_string(), &'ㅏ'));
/// assert!(!combine_status_check(&"가".to_string(), &'ㄱ'));
/// ```
pub fn combine_status_check(combined_one_char:&String, one_char: &char) -> bool {
    if !check_korean(*one_char) {
        return false;
    }

    // check the combined_one_char and combine in the result
    match combined_one_char.chars().count() {
        0 => first_letter_check(*one_char),
        1 => middle_letter_check(*one_char),
        2 => last_letter_check(*one_char),
        _ => panic!("cannot exceed 3 letters"),
    }
}

/// Combines Korean characters (choseong, jungseong, jongseong) into a single syllable.
///
/// # Arguments
/// * `combine_string` - A `String` containing up to three characters: initial consonant (choseong),
///   medial vowel (jungseong), and final consonant (jongseong) to combine into a single syllable.
///
/// # Returns
/// * `char` - A single Korean syllable formed by combining the given characters.
///
/// # Panics
/// * The function panics if the input string contains more than three characters or if the combination
///   cannot form a valid Korean syllable.
///
/// # Examples
/// ```
/// use rustkorean::compose_korean::make_one_letter;
///
/// assert_eq!(make_one_letter("가".to_string()), '가');
/// assert_eq!(make_one_letter("ㄱㅏ".to_string()), '가');
/// ```
pub fn make_one_letter(combine_string:String) -> char {
    // if need to combine only one, it means return itself.
    if combine_string.chars().count() == 1 {
        return combine_string.clone().pop().unwrap()
    }

    let cho = ['ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ'];
    let jung = ['ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ', 'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ'];
    let jong = [' ', 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ', 'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ'];

    let (mut cho_index, mut jung_index, mut jong_index) = (0, 0, 0);

    for (index, ch) in combine_string.chars().enumerate() {
        match index {
            0 => cho_index = cho.iter().position(|&c| c == ch).unwrap_or_default(),
            1 => jung_index = jung.iter().position(|&c| c == ch).unwrap_or_default(),
            2 => jong_index = jong.iter().position(|&c| c == ch).unwrap_or_default(),
            _ => panic!("Invalid input"),
        }
    }

    let result_code = 44032 + (cho_index * 588) + (jung_index * 28) + jong_index;
    std::char::from_u32(result_code as u32).unwrap_or_else(|| panic!("Invalid Hangul character"))
}