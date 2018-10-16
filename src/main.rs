use std::env;
mod java_doc_generator;
mod checkstyle_fix_spaces;
mod file_manager;
mod command_line_adapter;

fn main() {
    
    
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
       command_line_adapter::maven_check_style(String::from(&args[1][0..]));
    }
    else {
        println!("No File path has been given as program argument.");
    }

}
