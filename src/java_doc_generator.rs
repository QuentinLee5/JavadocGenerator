use file_manager;

pub fn fix_javadoc_all_files(files: &Vec<String>) {
    for file in files {
        file_manager::write_file(&file[0..], generate_javadoc(&file_manager::read_file(&file[0..])[0..]));
    }
}

/// Generates Javadoc for the given code.
fn generate_javadoc(code: &str) -> String {
    
    let lines = code.lines();
    
    let mut res = String::from("");
    let mut has_javadoc = false;
    for line in lines {
        if !has_javadoc && line.contains("get") && line.contains("public") {
            let field_name = get_field_name(line);
            let description = format!("     * The getter for the {} field of this class.\n", field_name);
            let return_text = format!("     * @return returns the value of the {} field of this class.\n", field_name);
            res.push_str(&String::from("    /**\n"));
            res.push_str(&description);
            res.push_str(&return_text);
            res.push_str(&String::from("     */\n"));
        }
        if !has_javadoc && line.contains("set") && line.contains("public"){
            let field_name = get_param_name(line).to_string();
            let description = format!("     * The setter for the {} field of this class.\n", field_name);
            let param_description = format!("     * @param {} The new value assigned to the {} field of the object.\n", field_name, field_name);
            res.push_str(&String::from("    /**\n"));
            res.push_str(&description);
            res.push_str(&param_description);
            res.push_str(&String::from("     */\n"));
        }
        
        res.push_str(&line.to_string());        
        res.push_str("\n");
        has_javadoc = false;
        if line.contains("*/") {
            has_javadoc = true;
        }
    }

    return res;    
}

/// Returns the name of the field given a line code.
fn get_field_name(line: &str) -> String {
    let index_1 = line.find("get").unwrap() + 3;
    let index_2 = line.find("()").unwrap();

    let char_vec:Vec<char> = line.chars().collect();
    let ch = char_vec[index_1];
    let mut res: String = String::from(ch.to_lowercase().to_string());
    res.push_str(&line[index_1 + 1..index_2]);

    res
}

/// Returns the name of the field using the parameter name given a line of code.
fn get_param_name(line: &str) -> &str {
    let first_parentheses = line.find('(').unwrap() + 1;
    let second_parentheses = line.find(')').unwrap();
    let words = &line[first_parentheses..second_parentheses];
    let splitted: Vec<&str> = words.split(' ').collect();
    splitted[1]

}

#[cfg(test)]
mod tests {        
    use super::*;

    #[test]
    fn test_field_name() {
        assert_eq!(get_field_name("public int getScore() {"), "score");
    }

    #[test]
    fn test_param_name() {
        assert_eq!(get_param_name("public void setScore(int score) {"), "score");
    }
}
