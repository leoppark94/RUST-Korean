use {first_letter_check, check_korean};

pub fn compose_korean(chars_vec: Vec<char>) -> String {

    // where to save the final result
    let mut result = String::new();
    // making the temp one korean char
    let mut combined_one_char = String::new();

    for one_char in chars_vec {
        if combine_status_check(&combined_one_char, &one_char) {
            println!("ㅇㅇ");
        }
        println!("{}", one_char);
        println!("{:?}", result.len());
    }

    String::new()
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
            1 => {}
            2 => {}
            3 => {}
            _ => {}
        }

    } else {
        false
    }
}