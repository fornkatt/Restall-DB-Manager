// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(test)]
mod tests;

use std::fmt;

/// Leaves room for the longest game names on the `RenoDX` wiki (up to 99 characters as an id).
///
/// Lowering it can invalidate ids that are already in the database.
const MAX_LENGTH: usize = 100;

/// Lowercase ASCII slug, e.g. `silent-hill-townfall`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModId(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModIdProblem {
    Empty,
    TooLong { max: usize },
    InvalidCharacter,
    StartsOrEndsWithHyphen,
    DoubleHyphen,
}

impl ModId {
    pub fn parse(text: &str) -> Result<ModId, ModIdProblem> {
        if text.is_empty() {
            return Err(ModIdProblem::Empty);
        }
        if !text.chars().all(is_allowed_character) {
            return Err(ModIdProblem::InvalidCharacter);
        }
        if text.len() > MAX_LENGTH {
            return Err(ModIdProblem::TooLong { max: MAX_LENGTH });
        }
        if text.starts_with('-') || text.ends_with('-') {
            return Err(ModIdProblem::StartsOrEndsWithHyphen);
        }
        if text.contains("--") {
            return Err(ModIdProblem::DoubleHyphen);
        }
        Ok(ModId(text.to_string()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

fn is_allowed_character(character: char) -> bool {
    matches!(character, 'a'..='z' | '0'..='9' | '-')
}
