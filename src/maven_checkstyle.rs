fn fix_checkstyle(output: String) -> String {
    find_error_lines(output);
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
