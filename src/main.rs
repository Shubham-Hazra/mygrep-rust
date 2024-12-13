#[allow(unused_imports)]

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.contains('|') {
        return match_alternation(input_line, pattern);
    }

    if pattern.starts_with('^') && pattern.ends_with('$') {
        let pattern_core = &pattern[1..pattern.len() - 1];
        return input_line.len() == pattern_core.len()
            && match_from_start(input_line, pattern_core);
    } else if pattern.starts_with('^') {
        return match_from_start(input_line, &pattern[1..]);
    } else if pattern.ends_with('$') {
        let pattern_core = &pattern[..pattern.len() - 1];
        return match_from_start(
            &input_line[input_line.len().saturating_sub(pattern_core.len())..],
            pattern_core,
        );
    } else {
        for start in 0..=input_line.len() {
            if match_from_start(&input_line[start..], pattern) {
                return true;
            }
        }
    }
    false
}

fn match_alternation(input_line: &str, pattern: &str) -> bool {
    if let Some(start) = pattern.find('(') {
        if let Some(end) = pattern[start..].find(')') {
            let alternation_block = &pattern[start + 1..start + end];
            let alternatives: Vec<&str> = alternation_block.split('|').collect();

            for alternative in alternatives {
                let new_pattern = format!(
                    "{}{}{}",
                    &pattern[..start],
                    alternative,
                    &pattern[start + end + 1..]
                );
                if match_pattern(input_line, &new_pattern) {
                    return true;
                }
            }
            return false;
        }
    }
    panic!("Invalid pattern: missing parentheses for alternation");
}

fn match_from_start(input_line: &str, pattern: &str) -> bool {
    let mut pat = pattern;
    let mut input = input_line;

    while !pat.is_empty() && !input.is_empty() {
        let (cur_pat, pat_len) = match parse_pattern(pat) {
            Some(parsed) => parsed,
            None => return false,
        };
        pat = &pat[pat_len..];

        if pat.starts_with('+') {
            if !consume_repeated(&mut input, &cur_pat) {
                return false;
            }
            pat = &pat[1..];
        } else if pat.starts_with('?') {
            consume_optional(&mut input, &cur_pat);
            pat = &pat[1..];
        } else {
            if !consume_single(&mut input, &cur_pat) {
                return false;
            }
        }
    }
    pat.is_empty()
}

fn parse_pattern(pat: &str) -> Option<(&str, usize)> {
    if pat.starts_with('\\') {
        if pat.len() < 2 {
            return None;
        }
        Some((&pat[..2], 2))
    } else if pat.starts_with('[') {
        if let Some(end_index) = pat.find(']') {
            Some((&pat[..=end_index], end_index + 1))
        } else {
            None
        }
    } else {
        Some((&pat[..1], 1))
    }
}

fn consume_repeated(input: &mut &str, pattern: &str) -> bool {
    if let Some(first_char) = input.chars().next() {
        if match_char(&first_char, pattern) {
            while let Some(next_char) = input.chars().next() {
                if !match_char(&next_char, pattern) {
                    break;
                }
                *input = &input[1..];
            }
            return true;
        }
    }
    false
}

fn consume_optional(input: &mut &str, pattern: &str) {
    if let Some(first_char) = input.chars().next() {
        if match_char(&first_char, pattern) {
            *input = &input[1..];
        }
    }
}

fn consume_single(input: &mut &str, pattern: &str) -> bool {
    if let Some(first_char) = input.chars().next() {
        if match_char(&first_char, pattern) {
            *input = &input[1..];
            return true;
        }
    }
    false
}

fn match_char(chr: &char, pattern: &str) -> bool {
    match pattern {
        "\\d" => chr.is_ascii_digit(),
        "\\w" => chr.is_ascii_alphanumeric(),
        "." => true,
        pat if pat.starts_with("[^") => {
            let chars: Vec<char> = pat[2..pat.len() - 1].chars().collect();
            !chars.contains(chr)
        }
        pat if pat.starts_with("[") => {
            let chars: Vec<char> = pat[1..pat.len() - 1].chars().collect();
            chars.contains(chr)
        }
        _ => pattern.len() == 1 && pattern.chars().next().unwrap() == *chr,
    }
}

fn run_tests() {
    let tests = vec![
        ("^hello$", "hello", true),
        ("^hello$", "helloo", false),
        ("^abc", "abcdef", true),
        ("abc$", "abcdef", false),
        ("abc$", "abc", true),
        ("a.b", "a1b", true),
        ("a.b", "ab", false),
        ("\\d+", "12345", true),
        ("\\d+", "abc123", true),
        ("(a|b)", "b", true),
        ("(a|b)", "c", false),
        ("(a|b)c", "ac", true),
        ("(a|b)c", "bc", true),
        ("a+bc", "aaabc", true),
        ("a?bc", "abc", true),
    ];

    for (pattern, input, expected) in tests {
        let result = match_pattern(input, pattern);
        if result == expected {
            println!("PASS: Pattern '{}' with input '{}' passed", pattern, input);
        } else {
            println!(
                "FAIL: Pattern '{}' with input '{}' failed. Expected: {}, Got: {}",
                pattern, input, expected, result
            );
        }
    }
}

fn main() {
    run_tests();
}
