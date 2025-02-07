pub fn split_command(input: &str) -> Vec<String> {
    let mut iter = input.trim().chars().peekable();

    let mut word = String::new();
    let mut result = Vec::new();

    let mut in_single_quote = false;
    let mut in_double_quote = false;

    while let Some(c) = iter.next() {
        match c {
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
                continue;
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
                continue;
            }
            '\\' if !in_single_quote && !in_double_quote => {
                let c = iter.next().unwrap();

                word.push(c);
                continue;
            }
            '\\' if in_double_quote => {
                match iter.peek().unwrap() {
                    '\\' | '$' | '"' => {
                        word.push(iter.next().unwrap());
                    }

                    _ => word.push(c),
                };
                continue;
            }
            ' ' if !in_single_quote && !in_double_quote => {
                if !word.is_empty() {
                    result.push(word.clone());
                    word.clear();
                }
                continue;
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
