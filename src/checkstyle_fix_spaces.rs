/// Fix the checkstyle errors which are caused by missing spaces.
pub fn fix_spaces(content: String) -> String {
    let mut result = String::from("");

    let lines = content.lines();

    for line in lines {
        if line_with_no_spaces(line) {
            result.push_str(line);
        }
        else {
            let chars:Vec<char> = line.chars().collect(); 

            for (index, value) in chars.iter().enumerate() {
                let char2 = {
                    if chars.len() > index + 1 {
                        chars[index + 1]
                    }
                    else {
                        ' '
                    }
                };
                let char3 = {
                    if index > 0 {
                        chars[index - 1]
                    }
                    else {
                        ' '
                    }
                };
                if *value == ',' && char2 != ' ' {
                    result.push(*value);
                    result.push(' ');              
                } 
                else if char_with_spaces(*value) &&  !valid_no_space(char3, char2) {
                    if char3 != ' ' {
                        result.push(' ');
                    }

                    result.push(*value);

                    if chars.len() > index + 1 && chars[index + 1] != ' ' {
                        result.push(' ');
                    }
                }
                else {
                    result.push(*value);
                }
            } 
        }
        result.push('\n');
    }

    result
}

/// Checks if the char is a char which must have a space as prefix and suffix.
fn char_with_spaces(input: char) -> bool {
    if input == '/' || input == '-' || input == '+' || input == '*' || input == '}' || input == '{' || input == '=' {
        return true;
    }
    false
}

/// Checks if the line contains a char which must have a space as prefix and suffix.
fn line_with_no_spaces(input: &str) -> bool {
    if input.contains('/') || input.contains('-') || input.contains('+') || input.contains('*') || input.contains('}') || input.contains('{') || input.contains('=') {
        return false;
    }
    true
}

/// Checks if the missing space is valid, because it is Javadoc.
fn valid_no_space(char_before: char, char_after: char) -> bool {
    if char_with_spaces(char_before) || char_with_spaces(char_after) || char_after == '>'{
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_with_space() {
        assert_eq!(char_with_spaces('{'), true);
    }

    #[test]
    fn test_char_with_no_space() {
        assert_eq!(char_with_spaces('w'), false);
    }

    #[test]
    fn test_valid_no_space_true() {
        assert_eq!(valid_no_space('/', ' '), true);
    }

    #[test]
    fn test_valid_no_space_false() {
        assert_eq!(valid_no_space('w', 'x'), false);
    }

    #[test]
    fn test_spaces_on_string_arithmetics() {
        assert_eq!(fix_spaces(String::from("2* 3 +4-z/ 3")), String::from("2 * 3 + 4 - z / 3\n"));
    }

    #[test]
    fn test_spaces_on_string_curly_braces() {
        assert_eq!(fix_spaces(String::from("public int test_method(){  }")), String::from("public int test_method() {  }\n"))
    }
}
