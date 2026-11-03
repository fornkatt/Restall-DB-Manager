// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(test)]
mod tests;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameName(String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Maintainer(String);

/// Free text that may span several lines.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Notes(String);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextProblem {
    Empty,
    SurroundingWhitespace,
    ControlCharacter,
    InvisibleCharacter,
}

impl GameName {
    pub fn parse(text: &str) -> Result<GameName, TextProblem> {
        check_single_line_text(text)?;
        Ok(GameName(text.to_string()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Names with equal keys count as the same name. Letter case is ignored.
    pub fn comparison_key(&self) -> String {
        self.0.to_lowercase()
    }
}

impl Maintainer {
    pub fn parse(text: &str) -> Result<Maintainer, TextProblem> {
        check_single_line_text(text)?;
        Ok(Maintainer(text.to_string()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Notes {
    pub fn parse(text: &str) -> Result<Notes, TextProblem> {
        check_multi_line_text(text)?;
        Ok(Notes(text.to_string()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for GameName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl fmt::Display for Maintainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl fmt::Display for Notes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

fn check_single_line_text(text: &str) -> Result<(), TextProblem> {
    check_not_empty_or_padded(text)?;
    if text.chars().any(char::is_control) {
        return Err(TextProblem::ControlCharacter);
    }
    check_no_invisible_characters(text)
}

fn check_multi_line_text(text: &str) -> Result<(), TextProblem> {
    check_not_empty_or_padded(text)?;
    if text
        .chars()
        .any(|character| character.is_control() && character != '\n')
    {
        return Err(TextProblem::ControlCharacter);
    }
    check_no_invisible_characters(text)
}

fn check_not_empty_or_padded(text: &str) -> Result<(), TextProblem> {
    if text.trim().is_empty() {
        return Err(TextProblem::Empty);
    }
    if text.trim() != text {
        return Err(TextProblem::SurroundingWhitespace);
    }
    Ok(())
}

fn check_no_invisible_characters(text: &str) -> Result<(), TextProblem> {
    if text.chars().any(is_invisible_format_character) {
        return Err(TextProblem::InvisibleCharacter);
    }
    Ok(())
}

/// Characters an editor shows as nothing, or that silently reorder the surrounding text.
fn is_invisible_format_character(character: char) -> bool {
    matches!(
        character,
        '\u{200B}'..='\u{200F}'
            | '\u{202A}'..='\u{202E}'
            | '\u{2060}'..='\u{2064}'
            | '\u{2066}'..='\u{2069}'
            | '\u{FEFF}'
    )
}
