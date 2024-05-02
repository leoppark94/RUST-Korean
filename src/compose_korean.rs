use ::{check_korean, first_letter_check, last_letter_check, middle_letter_check};

pub fn compose_korean(chars_vec: Vec<char>) -> String {
    // where to save the final result
    let mut result = String::new();
    // making the temp one korean char
    let mut combined_one_char = String::new();

    for one_char in chars_vec {
        // maximum length check
        if combined_one_char.chars().count() == 3 {
            result.push(make_one_letter(combined_one_char.clone()));
            combined_one_char.clear();
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

    result
}

pub fn combine_status_check(combined_one_char:&String, one_char: &char) -> bool {
    // check_one_char is Korean
    if check_korean(one_char.clone()) {
        // check the combined_one_char and combine in the result
        match combined_one_char.chars().count() {
            0 => {
                // check it's possible to used as a first letter
                if first_letter_check(one_char.clone()) {
                    true
                } else {
                    false
                }
            }
            1 => {
                if middle_letter_check(one_char.clone()) {
                    true
                } else {
                    false
                }
            }
            2 => {
                if last_letter_check(one_char.clone()) {
                    true
                } else {
                    false
                }
            }
            _ => { panic!("cannot exceed 3 letters")}
        }

    } else {
        false
    }
}

fn make_one_letter(combine_string:String) -> char {
    let cho = ['ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ'];
    let jung = ['ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ', 'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ'];
    let jong = [' ', 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ', 'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ'];

    let mut cho_index = 0;
    let mut jung_index = 0;
    let mut jong_index = 0;

    let mut result = 44032;

    for (index, ch) in combine_string.chars().enumerate() {
        match index {
            0 => cho_index = cho.iter().position(|&c| c == ch).unwrap_or_default(),
            1 => jung_index = jung.iter().position(|&c| c == ch).unwrap_or_default(),
            2 => jong_index = jong.iter().position(|&c| c == ch).unwrap_or_default(),
            _ => panic!("Invalid input"),
        }
    }

    let result = 44032 + (cho_index * 588) + (jung_index * 28) + jong_index;
    std::char::from_u32(result as u32).unwrap_or_else(|| panic!("Invalid Hangul character"))
}