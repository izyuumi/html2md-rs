use std::borrow::Cow;

pub(crate) fn decode_html_entities(input: &str) -> Cow<'_, str> {
    let mut output: Option<String> = None;
    let mut copied_until = 0;

    for (start, _) in input.match_indices('&') {
        let Some(end_offset) = input[start + 1..].find(';') else {
            continue;
        };
        let end = start + end_offset + 1;
        let entity = &input[start + 1..end];
        let decoded = match entity {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            "nbsp" => Some('\u{a0}'),
            numeric if numeric.starts_with("#x") || numeric.starts_with("#X") => {
                u32::from_str_radix(&numeric[2..], 16)
                    .ok()
                    .and_then(char::from_u32)
            }
            numeric if numeric.starts_with('#') => {
                numeric[1..].parse::<u32>().ok().and_then(char::from_u32)
            }
            _ => None,
        };

        if let Some(decoded) = decoded {
            let output = output.get_or_insert_with(|| String::with_capacity(input.len()));
            output.push_str(&input[copied_until..start]);
            output.push(decoded);
            copied_until = end + 1;
        }
    }

    match output {
        Some(mut output) => {
            output.push_str(&input[copied_until..]);
            Cow::Owned(output)
        }
        None => Cow::Borrowed(input),
    }
}

pub(crate) fn escape_markdown(input: &str) -> Cow<'_, str> {
    let mut output: Option<String> = None;
    let mut copied_until = 0;

    for (index, character) in input.char_indices() {
        if matches!(
            character,
            '\\' | '&'
                | '<'
                | '`'
                | '*'
                | '_'
                | '['
                | ']'
                | '('
                | ')'
                | '#'
                | '+'
                | '-'
                | '.'
                | '!'
                | '>'
                | '|'
                | '~'
        ) {
            let output = output.get_or_insert_with(|| String::with_capacity(input.len() + 1));
            output.push_str(&input[copied_until..index]);
            output.push('\\');
            output.push(character);
            copied_until = index + character.len_utf8();
        }
    }

    match output {
        Some(mut output) => {
            output.push_str(&input[copied_until..]);
            Cow::Owned(output)
        }
        None => Cow::Borrowed(input),
    }
}

pub(crate) fn escape_html_attribute(input: &str) -> Cow<'_, str> {
    let mut output: Option<String> = None;
    let mut copied_until = 0;

    for (index, character) in input.char_indices() {
        let escaped = match character {
            '&' => "&amp;",
            '"' => "&quot;",
            '<' => "&lt;",
            '>' => "&gt;",
            _ => continue,
        };
        let output = output.get_or_insert_with(|| String::with_capacity(input.len() + 4));
        output.push_str(&input[copied_until..index]);
        output.push_str(escaped);
        copied_until = index + character.len_utf8();
    }

    match output {
        Some(mut output) => {
            output.push_str(&input[copied_until..]);
            Cow::Owned(output)
        }
        None => Cow::Borrowed(input),
    }
}
