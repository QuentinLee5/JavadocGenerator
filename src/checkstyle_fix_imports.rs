use file_manager;

pub fn fix_all_files(files: &Vec<String>) {
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

pub fn fix_unused_import(file: &String, line_number: i32) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_delete_line() {
       let input = "line 1 \n||delete|| \nline 3";
       let res = String::from("line 1 \nline 3\n");
       assert_eq!(clean_file(String::from(input)), res);
    }
}
