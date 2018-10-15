use std::fs;

pub fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    contents
}

pub fn write_file(path: &str, content: String) {
    fs::write(path, content).expect("Something went wrong writing the file");
}
