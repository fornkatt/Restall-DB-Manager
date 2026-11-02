// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use super::*;

mod mod_id {
    use super::*;

    #[test]
    fn lowercase_slug_is_valid() {
        let id = ModId::parse("trails-of-cold-steel-3").unwrap();

        assert_eq!(id.as_str(), "trails-of-cold-steel-3");
    }

    #[test]
    fn empty_text_is_invalid() {
        assert_eq!(ModId::parse(""), Err(ModIdProblem::Empty));
    }

    #[test]
    fn one_hundred_characters_are_valid() {
        let text = "a".repeat(100);
        let result = ModId::parse(&text);

        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn one_hundred_and_one_characters_are_too_long() {
        let text = "a".repeat(101);

        assert_eq!(ModId::parse(&text), Err(ModIdProblem::TooLong { max: 100 }));
    }

    #[test]
    fn uppercase_letter_is_invalid() {
        assert_eq!(ModId::parse("Control"), Err(ModIdProblem::InvalidCharacter));
    }

    #[test]
    fn leading_hyphen_is_invalid() {
        assert_eq!(
            ModId::parse("-control"),
            Err(ModIdProblem::StartsOrEndsWithHyphen)
        );
    }

    #[test]
    fn double_hyphen_is_invalid() {
        assert_eq!(
            ModId::parse("control--resonant"),
            Err(ModIdProblem::DoubleHyphen)
        );
    }

    #[test]
    fn long_non_ascii_text_is_an_invalid_character() {
        let text = "ä".repeat(51);

        assert_eq!(ModId::parse(&text), Err(ModIdProblem::InvalidCharacter));
    }

    #[test]
    fn display_with_padding_returns_the_padded_id() {
        let value = ModId::parse("control-resonant").unwrap();

        assert_eq!(format!("{value:*<19}"), "control-resonant***");
    }
}
