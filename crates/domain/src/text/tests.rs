// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use super::*;

mod game_name {
    use super::*;

    #[test]
    fn unicode_name_is_valid() {
        let name = GameName::parse("NieR:Automata™").unwrap();

        assert_eq!(name.as_str(), "NieR:Automata™");
    }

    #[test]
    fn whitespace_only_is_empty() {
        assert_eq!(GameName::parse("   "), Err(TextProblem::Empty));
    }

    #[test]
    fn surrounding_space_is_invalid() {
        assert_eq!(
            GameName::parse(" Control"),
            Err(TextProblem::SurroundingWhitespace)
        );
    }

    #[test]
    fn line_break_is_invalid() {
        assert_eq!(
            GameName::parse("Con\ntrol"),
            Err(TextProblem::ControlCharacter)
        );
    }

    #[test]
    fn zero_width_space_is_invalid() {
        assert_eq!(
            GameName::parse("Control\u{200B}"),
            Err(TextProblem::InvisibleCharacter)
        );
    }

    #[test]
    fn display_with_padding_returns_the_padded_text() {
        let value = GameName::parse("Control").unwrap();

        assert_eq!(format!("{value:*<10}"), "Control***");
    }
}
mod maintainer {
    use super::*;

    #[test]
    fn name_with_parentheses_is_valid() {
        let maintainer = Maintainer::parse("John (Rust)").unwrap();

        assert_eq!(maintainer.as_str(), "John (Rust)");
    }

    #[test]
    fn line_break_is_invalid() {
        assert_eq!(
            Maintainer::parse("John\nRust"),
            Err(TextProblem::ControlCharacter)
        );
    }

    #[test]
    fn display_with_padding_returns_the_padded_text() {
        let value = Maintainer::parse("John (Rust)").unwrap();

        assert_eq!(format!("{value:*<14}"), "John (Rust)***");
    }
}

mod notes {
    use super::*;

    #[test]
    fn multi_line_notes_are_valid() {
        let notes = Notes::parse("Upgrade Path: On\nLacks testing.").unwrap();

        assert_eq!(notes.as_str(), "Upgrade Path: On\nLacks testing.");
    }

    #[test]
    fn tab_is_invalid() {
        assert_eq!(
            Notes::parse("Upgrade\tPath"),
            Err(TextProblem::ControlCharacter)
        );
    }

    #[test]
    fn right_to_left_override_is_invalid() {
        assert_eq!(
            Notes::parse("Works \u{202E}fine"),
            Err(TextProblem::InvisibleCharacter)
        );
    }

    #[test]
    fn display_with_padding_returns_the_padded_text() {
        let value = Notes::parse("Upgrade Path: On\nLacks testing.").unwrap();

        assert_eq!(
            format!("{value:*<34}"),
            "Upgrade Path: On\nLacks testing.***"
        );
    }
}
