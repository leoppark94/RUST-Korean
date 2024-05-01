use ::{check_korean, first_letter_check, last_letter_check, middle_letter_check};

pub fn compose_korean(chars_vec: Vec<char>) -> String {

    // where to save the final result
    let mut result = String::new();
    // making the temp one korean char
    let mut combined_one_char = String::new();

    for one_char in chars_vec {
        if combined_one_char.len() > 2 {
            result.push(make_one_letter());
            combined_one_char.clear();
        }

        if combine_status_check(&combined_one_char, &one_char) {
            combined_one_char.push(one_char);
        } else {
            if combined_one_char.len() != 0 {
                result.push(make_one_letter());
                combined_one_char.clear();
            }
            result.push(one_char);
        }
    }

    result
}

pub fn combine_status_check(combined_one_char:&String, one_char: &char) -> bool {
    // check_one_char is Korean
    if check_korean(one_char.clone()) {
        // check the combined_one_char and combine in the result
        match combined_one_char.len() {
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

fn make_one_letter() -> char {
    'ㅇ'
}