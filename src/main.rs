use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        _ if pattern.chars().count() == 1 => input_line.contains(pattern),
        "\\w" => input_line.chars().any(|c| c.is_alphanumeric()),
        "\\d" => input_line.chars().any(|c| c.is_ascii_digit()),
        _ if pattern.starts_with('[') && pattern.ends_with(']') => {
            let char_group = pattern
                .trim_matches(|c| c == '[' || c == ']')
                .chars()
                .collect::<Vec<char>>();
            input_line.chars().any(|c| char_group.contains(&c))
        }
        _ => panic!("Pattern not handled!: {}", pattern),
    }
}

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
