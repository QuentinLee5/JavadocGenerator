use std::env;
mod java_doc_generator;
mod checkstyle_fixer;
mod file_manager;
mod command_line_adapter;

fn main() {
    
    command_line_adapter::maven_check_style(String::from("/Users/quentinlee/Desktop/TI1216-17.0.1"));

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let path = &args[1];
        let content = file_manager::read_file(path);
        let result: String = java_doc_generator::generate_javadoc(&content);
        let result2 = checkstyle_fixer::fix_checkstyle(result);    
        file_manager::write_file(path, result2);
        println!("Generating javadoc and fixing checkstyle errors done");
    }
    else {
        println!("No File path has been given as program argument.");
    }

}
