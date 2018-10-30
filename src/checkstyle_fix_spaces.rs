use file_manager;
use regex::Regex;

pub fn fix_spaces_all_files(files: &Vec<String>) {
    for file in files {
        file_manager::write_file(&file[0..], fix_spaces(file_manager::read_file(&file[0..])));
    }
}

/// Fix the checkstyle errors which are caused by missing spaces.
fn fix_spaces(content: String) -> String {
    let re_before = Regex::new(r"(?P<before>[^!/\-+*}{=\s])(?P<c>[/\-+*}{=])").unwrap();
    let temp_res = re_before.replace_all(&content[..], "$before $c");
    let re_after = Regex::new(r"(?P<c>[/\-+*},{=])(?P<after>[^!/\-+*}{=\s>])").unwrap();
    let temp_content = String::from(temp_res);
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
        assert_eq!(fix_spaces(String::from("public int test_method(int x,int y){  }")), String::from("public int test_method(int x, int y) {  }"))
    }

    #[test]
    fn test_no_spaces_added_equals() {
        assert_eq!(fix_spaces(String::from("==")), String::from("=="));
    }

    #[test]
    fn test_no_space_added_plusses() {
        assert_eq!(fix_spaces(String::from("++")), String::from("++"));
    }

    #[test]
    fn test_javadoc_no_spaces_inserted() {
        assert_eq!(fix_spaces(String::from("/**\n*\n*\n*/")), String::from("/**\n*\n*\n*/"));
    }

    #[test]
    fn test_lambda() {
        assert_eq!(fix_spaces(String::from("x->{  }")), String::from("x -> {  }"));
    }

    #[test]
    fn test_not_equals_operator() {
        assert_eq!(fix_spaces(String::from("!=")), String::from("!=")); 
    }
}
