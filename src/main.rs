use std::fs;

fn main() {   
    let path = "/Users/quentinlee/Desktop/test.java";
    generate_javadoc(path);

}

fn generate_javadoc(path: &str) {
    let output = read_file(path);
    
    let lines = output.lines();
    
    let mut res = String::from("");
    let mut has_javadoc = false;
    for line in lines {
        if !has_javadoc && line.contains("get") && line.contains("public") {
            let field_name = get_field_name(line);
            let description = format!("     * The getter for the {} field of this class\n", field_name);
            let return_text = format!("     * @return returns the value of the {} field of this class\n", field_name);
            res.push_str(&String::from("    /**\n"));
            res.push_str(&description);
            res.push_str(&return_text);
            res.push_str(&String::from("     */\n"));
        }
        if !has_javadoc && line.contains("set") && line.contains("public"){
            let field_name = get_param_name(line).to_string();
            let description = format!("     * The setter for the {} field of this class\n", field_name);
            let param_description = format!("     * @param {} The new value assigned to the {} field of the object\n", field_name, field_name);
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

    println!("{}", res);
    
    fs::write(path, res).expect("adding javadoc to file did not succeed");

}

fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    contents
}

fn get_field_name(line: &str) -> String {
    let index_1 = line.find("get").unwrap() + 3;
    let index_2 = line.find("()").unwrap();
    
    let char_vec:Vec<char> = line.chars().collect();
    let ch = char_vec[index_1];
    let mut res: String = String::from(ch.to_lowercase().to_string());
    res.push_str(&line[index_1 + 1..index_2]);
    
    res
}

fn get_param_name(line: &str) -> &str {
    let first_parentheses = line.find('(').unwrap() + 1;
    let second_parentheses = line.find(')').unwrap(); 
    let words = &line[first_parentheses..second_parentheses];
    let splitted: Vec<&str> = words.split(' ').collect();
    splitted[1]

}
