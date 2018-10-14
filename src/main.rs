use std::fs;
use std::env;
mod java_doc_generator;
mod checkstyle_fixer;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        let path = &args[1];
        let content = read_file(path);
        let result: String = java_doc_generator::generate_javadoc(&content);
        let result2 = checkstyle_fixer::fix_checkstyle(result);    
        write_file(path, result2);
    }


}

fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    contents
}

fn write_file(path: &str, content: String) {
    fs::write(path, content).expect("Something went wrong reading the file");
}
