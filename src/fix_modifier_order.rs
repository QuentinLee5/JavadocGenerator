use file_manager;

pub fn fix_modifier_error(file: String, line_number: i32) {
    let content = file_manager::read_file(&file[..]);

    let mut res = String::from("");

    let lines = content.lines();

    let mut line_count = 0;

    for line in lines{
        line_count += 1;

        if line_count == line_number {
            res.push_str(&get_right_modifier_order(line.to_string()));
            res.push('\n');
            continue;
        }

        res.push_str(line);
        res.push('\n');
    }
    file_manager::write_file(&file[..], res);
}


enum Modifiers {
    Public,
    Private,
    Protected,
    Abstact,
    DefaultModifier,
    Static,
    Final,
    Transient,
    Volatile,
    Synchronized,
    Native,
    Strictfp,
    Unknown(String),
}

impl Modifiers {
    pub fn get_modifier_from_string(word: &str) -> Self {
        match &word[..] {
            "public" => Modifiers::Public,
            "private" => Modifiers::Private,
            "protected" => Modifiers::Protected,
            "abstract" => Modifiers::Abstact,
            "default" => Modifiers::DefaultModifier,
            "static" => Modifiers::Static,
            "final" => Modifiers::Final,
            "transient" => Modifiers::Transient,
            "volatile" => Modifiers::Volatile,
            "synchronized" => Modifiers::Synchronized,
            "native" => Modifiers::Native,
            "strictfp" => Modifiers::Strictfp,
            " " => Modifiers::Unknown(String::from("")),
            _ => {
                Modifiers::Unknown(String::from(word))
            }
        }
    }
}

fn get_right_modifier_order(line: String) -> String {
    let words: Vec<&str> = line.split(' ').collect();

    let mut res = String::from("");

    let chars: Vec<char> = line.chars().collect();

    for letter in chars {
        if letter == ' ' {
            res.push(' ');
        }
        else {
            break;
        }
    }
    if line.contains("public") {
        res.push_str("public ");
    }
    if line.contains("private") {
        res.push_str("private ");
    }
    if line.contains("protected") {
        res.push_str("protected ");
    }
    if line.contains("abstract") {
        res.push_str("abstract ");
    }
    if line.contains("default") {
        res.push_str("default ");
    }
    if line.contains("static") {
        res.push_str("static ");
    }
    if line.contains("final") {
        res.push_str("final ");
    }
    if line.contains("transient") {
        res.push_str("transient ");
    }
    if line.contains("volatile") {
        res.push_str("volatile ");
    }
    if line.contains("synchronized") {
        res.push_str("synchronized ");
    }
    if line.contains("native") {
        res.push_str("native ");
    }
    if line.contains("strictfp") {
        res.push_str("strictfp ");
    }
    for word in words {
        let modifier = Modifiers::get_modifier_from_string(word);
        match modifier {
            Modifiers::Unknown(word) => {
                res.push_str(&word[..]);
                if &word[..] != "" {
                    res.push_str(" ");
                }
            },
            _ => {}
        }

    }
    res
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_order_1() {
        let input = String::from("    static public int count = 3;");
        let expected = String::from("    public static int count = 3; ");

        assert_eq!(get_right_modifier_order(input), expected);
    }

    #[test]
    fn test_new_order_2() {
        let input = String::from("    final static private int score = 3;");
        let expected = String::from("    private static final int score = 3; ");

        assert_eq!(get_right_modifier_order(input), expected);
    }

    #[test]
    fn test_new_order_3() {
        let input = String::from("    abstract protected Object obj = new Object();");
        let expected = String::from("    protected abstract Object obj = new Object(); ");

        assert_eq!(get_right_modifier_order(input), expected);
    }

    #[test]
    fn test_new_order_4() {
        let input = String::from("    transient public Object obj = new Object();");
        let expected = String::from("    public transient Object obj = new Object(); ");

        assert_eq!(get_right_modifier_order(input), expected);
    }

    #[test]
    fn test_new_order_all_modifiers() {
        let input = String::from("    static default native strictfp default private public volatile abstract synchronized protected transient final Object obj = new Object();");
        let expected = String::from("    public private protected abstract default static final transient volatile synchronized native strictfp Object obj = new Object(); ");
            
        assert_eq!(get_right_modifier_order(input), expected);
    }
}


