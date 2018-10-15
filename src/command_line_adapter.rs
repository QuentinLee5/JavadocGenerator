use std::process::Command;
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

    let errors = find_error_lines(output_string.to_string());

    for error in errors {
        if get_error_from_message(&error) == "UnusedImports" {
            fix_unused_import(get_file_path(&error), get_line_number(&error));
        }
    }
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

fn fix_unused_import(file: String, line_number: i32) {
   let content = file_manager::read_file(&file[0..]); 

   let lines = content.lines();
    
    let mut result = String::from("");

   let mut line_count = 1;

   for line in lines {
        if line_count != line_number {
            result.push_str(line);
            result.push('\n');
        }
        line_count += 1;
   }

   file_manager::write_file(&file[0..], result);

   println!("Fixed unused import of file {} at line {}", file, line_number);
}
