use std::env;
mod java_doc_generator;

fn main() {  
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    java_doc_generator::generate_javadoc(&path);

}
