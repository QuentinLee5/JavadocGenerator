pub fn fix_checkstyle(content: String) -> String {
    fix_spaces(content)
}

fn fix_spaces(content: String) -> String {
    let mut result = String::from("");

    let chars:Vec<char> = content.chars().collect(); 

    for (index, value) in chars.iter().enumerate() {
        if *value == ',' && chars[index + 1] != ' ' {
            result.push(*value);
            result.push(' ');              
        } 
        else if char_with_spaces(*value) && !valid_no_space(chars[index - 1], chars[index + 1]) {
            if chars[index - 1] != ' ' {
                result.push(' ');
            }

            result.push(*value);
            
            if chars[index + 1] != ' ' {
                result.push(' ');
            }
        }
        else {
            result.push(*value);
        }
    }
    
    result
}

fn char_with_spaces(input: char) -> bool {
    if input == '/' || input == '-' || input == '+' || input == '*' {
        return true;
    }
    false
}

fn valid_no_space(char_before: char, char_after: char) -> bool {
    if char_with_spaces(char_before) || char_with_spaces(char_after) {
        return true;
    }
    false
}
