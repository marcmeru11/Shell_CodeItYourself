pub fn split_command(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut word = String::new();

    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            let c = chars.next().unwrap();

            word.push(c);
        } else if c == '\'' && !in_double_quotes {
            in_single_quotes = !in_single_quotes;
        } else if c == '"' && !in_single_quotes {
            in_double_quotes = !in_double_quotes;
        } else if c == ' ' && !in_single_quotes && !in_double_quotes {
            if !word.is_empty() {
                result.push(word.clone()); // Guardar palabra actual
                word.clear();
            }
        } else {
            word.push(c);
        }
    }

    if !word.is_empty() {
        result.push(word);
    }

    result
}
