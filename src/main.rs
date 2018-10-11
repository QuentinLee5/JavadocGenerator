use std::fs;

fn main() {   
    let path = "/Users/quentinlee/Desktop/test.java";
    generate_javadoc(path);

}

fn generate_javadoc(path: &str) {
    let output = read_file(path);
    
    let lines = output.lines();
    
    let mut res = Vec::new();
    for (index, line) in lines.enumerate() {
        if line.contains("get") {
            res.push("    /**");
            res.push("     * getter for this field");
            res.push("     * @return ");
            res.push("     */");
        }
        if line.contains("set") {
            let field_name = getParamName(line).to_string();
            let description = format!("The setter for the {} field of this class", field_name).to_owned();
            let description = "The setter for the ";
            description.push("test");
            res.push("    /**");
            res.push(&description);
            res.push("     */");
        }

        res.push(line);        
    }

    for line in res {
        println!("{}", line);
    }


}

fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    return contents;
}

fn getParamName(line: &str) -> &str {
    let first_parentheses = line.find("(").unwrap() + 1;
    let second_parentheses = line.find(")").unwrap(); 
    let words = &line[first_parentheses..second_parentheses];
    let splitted: Vec<&str> = words.split(' ').collect();
    return splitted[1];

}
