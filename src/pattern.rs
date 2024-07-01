use std::str::Chars;

#[derive(Debug)]
pub enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    PositiveCharGroup(String),
    NegativeCharGroup(String),
    StartOfLine,
    EndOfLine,
}

impl Pattern {
    pub fn match_literal(chars: &mut Chars, literal: char) -> bool {
        match chars.next() {
            Some(c) if c == literal => true,
            _ => false,
        }
    }

    pub fn match_digit(chars: &mut Chars) -> bool {
        match chars.next() {
            Some(c) if c.is_ascii_digit() => true,
            _ => false,
        }
    }

    pub fn match_alphanumeric(chars: &mut Chars) -> bool {
        match chars.next() {
            Some(c) if c.is_alphanumeric() => true,
            _ => false,
        }
    }

    pub fn match_positive_char_group(chars: &mut Chars, char_group: &str) -> bool {
        match chars.next() {
            Some(c) if char_group.contains(c) => true,
            _ => false,
        }
    }

    pub fn match_negative_char_group(chars: &mut Chars, char_group: &str) -> bool {
        match chars.next() {
            Some(c) if !char_group.contains(c) => true,
            _ => false,
        }
    }

    pub fn matches(&self, chars: &mut Chars, is_start: bool, is_end: bool) -> bool {
        match self {
            Pattern::StartOfLine => is_start,
            Pattern::EndOfLine => is_end && chars.clone().next().is_none(),
            Pattern::Literal(literal) => Pattern::match_literal(chars, *literal),
            Pattern::Digit => Pattern::match_digit(chars),
            Pattern::Alphanumeric => Pattern::match_alphanumeric(chars),
            Pattern::PositiveCharGroup(char_group) => {
                Pattern::match_positive_char_group(chars, char_group)
            }
            Pattern::NegativeCharGroup(char_group) => {
                Pattern::match_negative_char_group(chars, char_group)
            }
        }
    }
}
