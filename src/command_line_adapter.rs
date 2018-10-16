use std::process::Command;
use java_doc_generator;
use checkstyle_fix_spaces;
use file_manager;
pub fn maven_check_style(project_path: String) {
    println!("Running checkstyle...\n");
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "ls"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .current_dir(project_path)
            .arg("-c")
            .arg("mvn checkstyle:checkstyle")
            .output()
            .expect("failed to execute process")
    };

    let output_string = String::from_utf8_lossy(&output.stdout); 

    fix_checkstyle(&output_string.to_string());
}

fn fix_checkstyle(output: &String) {
    let errors_string = find_error_lines(output.to_string());

    let mut files_removed_lines: Vec<String> = Vec::new();

    let mut files_fix_spaces: Vec<String> = Vec::new();

    let mut files_fix_javadoc: Vec<String> = Vec::new();

    for error in errors_string {

        let file_path = get_file_path(&error);

        let actual_error = get_error_from_message(&error);

        let line_number = get_line_number(&error);

        if actual_error == "UnusedImports" {
            fix_unused_import(&file_path, line_number);
            if !contains_file(&files_removed_lines, &file_path) {
                files_removed_lines.push(String::from(&file_path[0..]));
            }
        }

        if actual_error.contains("Whitespace") {
            if !contains_file(&files_fix_spaces, &file_path) {
                files_fix_spaces.push(String::from(&file_path[0..]));
            }
        }

        if actual_error.contains("Javadoc") {
            if !contains_file(&files_fix_javadoc, &file_path) {
                files_fix_javadoc.push(String::from(&file_path[0..]));
            }
        }
    }

    clean_all_files(&files_removed_lines);

    fix_spaces_all_files(&files_fix_spaces);

    fix_javadoc_all_files(&files_fix_javadoc);
}

fn fix_javadoc_all_files(files: &Vec<String>) {
    for file in files {
        file_manager::write_file(&file[0..], java_doc_generator::generate_javadoc(&file_manager::read_file(&file[0..])[0..]));
    }
}

fn fix_spaces_all_files(files: &Vec<String>) {
    for file in files {
        file_manager::write_file(&file[0..], checkstyle_fix_spaces::fix_spaces(file_manager::read_file(&file[0..])));
    }
}

fn clean_all_files(files: &Vec<String>) {
    for file in files {
        let content = file_manager::read_file(&file[0..]);
        file_manager::write_file(&file[0..], clean_file(content));
        println!("Fix spaces of file {}", file);
    }
}

fn clean_file(content: String) -> String {
    let lines = content.lines();

    let mut result = String::from("");

    for line in lines {
        if !line.contains("||delete||") {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

fn contains_file(files: &Vec<String>, other: &String) -> bool {
    for file in files {
        if *file == *other {
            return true;
        }
    }
    false
}


fn find_error_lines(output: String) -> Vec<String> {
    let lines = output.lines();

    let mut error_lines = Vec::new();

    let mut is_audit = false;

    for line in lines {
        if line.contains("Audit done.") {
            break;
        }

        if is_audit {
            error_lines.push(String::from(line));
        }

        if line.contains("Starting audit") {
            is_audit = true;
        }
    }

    return error_lines;
}

fn get_error_from_message(message: &String) -> String {
    let actual_message = get_actual_message(message);

    let index = actual_message.find('[').unwrap() + 1;

    actual_message[index..actual_message.len() - 1].to_string()
}


fn get_actual_message(message: &String) -> String {
    let index = message.find(']').unwrap() + 2;

    String::from(&message[index..])
}

fn get_file_path(message: &String) -> String {
    let actual_message = get_actual_message(message);

    let index = actual_message.find(":").unwrap();

    String::from(&actual_message[..index])
}

fn get_line_number(message: &String) -> i32 {
    let actual_message = get_actual_message(message);

    let index_1 = actual_message.find(':').unwrap() + 1;

    let temp = String::from(&actual_message[index_1..]);

    let index_2 = temp.find(':').unwrap();

    let result = String::from(&temp[..index_2]);
    result.parse::<i32>().unwrap()
}

fn fix_unused_import(file: &String, line_number: i32) {
    let content = file_manager::read_file(&file[0..]); 

    let lines = content.lines();

    let mut result = String::from("");

    let mut line_count = 1;

    for line in lines {
        if line_count != line_number {
            result.push_str(line);
            result.push('\n');
        }
        else {
            result.push_str("||delete||\n");
        }
        line_count += 1;
    }

    file_manager::write_file(&file[0..], result);

    println!("Fixed unused import of file {} at line {}", file, line_number);
}
