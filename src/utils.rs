pub fn split_command(input: &str) -> Vec<String> {
    let mut iter = input.trim().chars().peekable();

    let mut word = String::new();
    let mut result = Vec::new();

    let mut in_single_quote = false;
    let mut in_double_quote = false;

    while let Some(c) = iter.next() {
        match c {
            '\'' => {
                if in_single_quote {
                    in_single_quote = false;
                } else if !in_double_quote {
                    in_single_quote = true;
                } else {
                    word.push(c);
                }
            }

            '"' => {
                if in_double_quote {
                    in_double_quote = false;
                } else if !in_single_quote {
                    in_double_quote = true;
                } else {
                    word.push(c);
                }
            }

            '\\' => {
                if in_single_quote || (in_double_quote && iter.peek() == Some(&'"')) {
                    word.push(c);
                } else if iter.peek() == Some(&'\\') || iter.peek() == Some(&'$') {
                    word.push(iter.next().unwrap());
                } else {
                    word.push(c);
                }
            }

            ' ' => {
                if in_single_quote || in_double_quote {
                    word.push(c);
                } else if !word.is_empty() {
                    result.push(word);
                    word = String::new();
                }
            }

            _ => {
                word.push(c);
            }
        }
    }
    result
}
