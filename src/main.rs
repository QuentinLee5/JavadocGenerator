use std::fs;
use std::env;
mod java_doc_generator;

fn main() {  
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let content = read_file(path);
    let result: String = java_doc_generator::generate_javadoc(&content);
    write_file(path, result);

}

fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    contents
}

fn write_file(path: &str, content: String) {
    fs::write(path, content).expect("Something went wrong reading the file");
}
