use crate::pattern::Pattern;

pub fn build_patterns(pattern: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut chars = pattern.chars();

    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some('d') => patterns.push(Pattern::Digit),
                Some('w') => patterns.push(Pattern::Alphanumeric),
                _ => patterns.push(Pattern::Literal(c)),
            },
            '[' => {
                let mut char_group = String::new();
                let mut negate = false;
                if let Some(c) = chars.next() {
                    if c == '^' {
                        negate = true;
                    } else {
                        char_group.push(c);
                    }
                }
                while let Some(c) = chars.next() {
                    if c == ']' {
                        break;
                    }
                    char_group.push(c);
                }
                if negate {
                    patterns.push(Pattern::NegativeCharGroup(char_group));
                } else {
                    patterns.push(Pattern::PositiveCharGroup(char_group));
                }
            }
            _ => patterns.push(Pattern::Literal(c)),
        }
    }

    patterns
}

pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let patterns = build_patterns(pattern);
    let mut chars = input_line.trim_end().chars();

    while chars.clone().next().is_some() {
        let mut char_pos = chars.clone();
        if patterns.iter().all(|p| p.matches(&mut char_pos)) {
            return true;
        }
        chars.next();
    }

    false
}
