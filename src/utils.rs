pub fn split_command(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut word = String::new();

    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut chars = input.chars().peekable();

    while let Some(mut c) = chars.next() {
        match c {
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            }
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            }
            '\\' if !in_single_quotes && !in_double_quotes => {
                c = chars.next().unwrap();
                word.push(c);
            }
            '\\' if in_double_quotes => {
                c = chars.next().unwrap();
                if c=='\\' || c =='$' || c=='\'' || c=='"' {
                    word.push(c);
                } else {
                    word.push('\\');
                }
            }
            ' ' if !in_single_quotes && !in_double_quotes => {
                if !word.is_empty() {
                    result.push(word.clone());
                    word = String::new();
                }
            }
            _ => {
                word.push(c);
            }
        }
    }

    if !word.is_empty() {
        result.push(word);
    }

    result
}
