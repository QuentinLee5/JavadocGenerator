use std::process::Command;

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
        println!("{}", get_error_from_message(error));
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

fn get_error_from_message(message: String) -> String {
    let actual_message = get_actual_message(message);
    
    let index = actual_message.find('[').unwrap() + 1;
    
    actual_message[index..actual_message.len() - 1].to_string()
}


fn get_actual_message(message: String) -> String {
    let index = message.find(']').unwrap();

    String::from(&message[index..])
}
