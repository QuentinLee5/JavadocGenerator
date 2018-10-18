use std::process::Command;
use java_doc_generator;
use checkstyle_fix_imports;
use checkstyle_fix_spaces;

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

    println!("Finished running checkstyle");

    let output_string = String::from_utf8_lossy(&output.stdout); 


    fix_checkstyle(&output_string.to_string());

    println!("Finished");
}

#[derive(Debug)]
enum Errors {
    UnusedImports(String, i32),
    WhiteSpace(String, i32),
    JavaDoc(String, i32),
    Unknown,
}

impl Errors {
    pub fn convert(err_str: &str) -> Self {
        let file_path = get_file_path(err_str);
        let actual_error = get_error_from_message(err_str);
        let line_number = get_line_number(err_str);
        match &actual_error[..] {
            "Imports" => Errors::UnusedImports(file_path, line_number),
            "WhiteSpace" => Errors::WhiteSpace(file_path, line_number),
            "Javadoc" => Errors::JavaDoc(file_path, line_number),
            _ => Errors::Unknown
        }
    }
}

fn fix_checkstyle(output: &String) {
    let errors_string = find_error_lines(output.to_string());

    let errors = errors_string
        .into_iter()
        .map(|i| Errors::convert(&i));
    let mut files_removed_lines: Vec<String> = Vec::new();

    let mut files_fix_spaces: Vec<String> = Vec::new();

    let mut files_fix_javadoc: Vec<String> = Vec::new();

    for error in errors {
        match error {
            Errors::UnusedImports(file, line_number) => {
                checkstyle_fix_imports::fix_unused_import(&file, line_number);
                if !files_removed_lines.contains(&file) {
                    files_removed_lines.push(file);
                }
            },
            Errors::WhiteSpace(file, _line_number) => {
                if !files_fix_spaces.contains(&file) {
                    files_fix_spaces.push(file);
                }
            },
            Errors::JavaDoc(file, _line_number) => {
                if !files_fix_javadoc.contains(&file) {
                    files_fix_javadoc.push(file)    
                }
            }, 
            Errors::Unknown => {}
        }
    }
    
    println!("Removing all unused javadoc");
    checkstyle_fix_imports::fix_all_files(&files_removed_lines);

    println!("Adding missing spaces");
    checkstyle_fix_spaces::fix_spaces_all_files(&files_fix_spaces);

    println!("Adding Javadoc to getters and setters");
    java_doc_generator::fix_javadoc_all_files(&files_fix_javadoc);
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

fn get_error_from_message(message: &str) -> String {
    let actual_message = get_actual_message(message);

    let index = actual_message.find('[').unwrap() + 1;

    let temp = actual_message[index..actual_message.len() - 1].to_string();

    if temp.contains("Javadoc") {
        return String::from("Javadoc");
    }
    if temp.contains("Whitespace") {
        return String::from("WhiteSpace");
    }
    if temp.contains("Import") {
        return String::from("Imports");
    }

    temp
}


fn get_actual_message(message: &str) -> String {
    let index = message.find(']').unwrap() + 2;

    String::from(&message[index..])
}

fn get_file_path(message: &str) -> String {
    let actual_message = get_actual_message(message);

    let index = actual_message.find(":").unwrap();

    String::from(&actual_message[..index])
}

fn get_line_number(message: &str) -> i32 {
    let actual_message = get_actual_message(message);

    let index_1 = actual_message.find(':').unwrap() + 1;

    let temp = String::from(&actual_message[index_1..]);

    let index_2 = temp.find(':').unwrap();

    let result = String::from(&temp[..index_2]);
    result.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_path() {
        let message = "[WARN] /src/main/java/grade/ProjectGrade.java:30:40: '%' is not followed by whitespace. [WhitespaceAround]";
        assert_eq!(get_file_path(message), String::from("/src/main/java/grade/ProjectGrade.java"));
    }

    #[test]
    fn test_get_line_number() {
       let message = "[WARN] /src/main/java/grade/ProjectGrade.java:30:40: '%' is not followed by whitespace. [WhitespaceAround]"; 
       assert_eq!(get_line_number(message), 30);
    } 

    #[test]
    fn test_get_error() {
        let message = "[WARN] /src/main/java/grade/ProjectGrade.java:30:40: '%' is not followed by whitespace. [WhitespaceAround]"; 
        assert_eq!(get_error_from_message(message), String::from("WhiteSpace"));
    }

    #[test]
    fn test_get_error_lines() {
        let input = String::from("test\ntest\nStarting audit\ntest\ntest\ntest\nAudit done.");
        assert_eq!(find_error_lines(input).len(), 3);
    }
}
