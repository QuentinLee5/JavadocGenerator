use file_manager;
use regex::Regex;

pub fn fix_spaces_all_files(files: &Vec<String>) {
    for file in files {
        file_manager::write_file(&file[0..], fix_spaces(file_manager::read_file(&file[0..])));
    }
}

/// Fix the checkstyle errors which are caused by missing spaces.
fn fix_spaces(content: String) -> String {
    let re_before = Regex::new(r"(?P<before>[^/\-+*}{=\s])(?P<c>[/\-+*}{=])").unwrap();
    let temp = re_before.replace_all(&content[..], "$before $c");
    let re_after = Regex::new(r"(?P<c>[/\-+*}{=])(?P<after>[^/\-+*}{=\s>])").unwrap();
    let temp_content = String::from(temp);
    let result = re_after.replace_all(&temp_content[..], "$c $after"); 
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spaces_on_string_arithmetics() {
        assert_eq!(fix_spaces(String::from("2* 3+4- z / 3")), String::from("2 * 3 + 4 - z / 3"));
    }

    #[test]
    fn test_spaces_on_string_curly_braces() {
        assert_eq!(fix_spaces(String::from("public int test_method(){  }")), String::from("public int test_method() {  }"))
    }

    #[test]
    fn test_javadoc_no_spaces_inserted() {
        assert_eq!(fix_spaces(String::from("/**\n*\n*\n*/")), String::from("/**\n*\n*\n*/"));
    }
}
