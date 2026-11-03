// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use super::*;

mod addon_64_url {
    use super::*;

    #[test]
    fn github_pages_addon_is_valid() {
        let url =
            Addon64Url::parse("https://fornkatt.github.io/renodx/renodx/renodx-control.addon64")
                .unwrap();

        assert_eq!(url.file_name(), "renodx-control.addon64");
    }

    #[test]
    fn uppercase_extension_is_valid() {
        let result = Addon64Url::parse("https://fornkatt.github.io/renodx/Renodx-X.ADDON64");

        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn addon_32_file_is_invalid() {
        assert_eq!(
            Addon64Url::parse("https://fornkatt.github.com/renodx/renodx-x.addon32"),
            Err(LinkProblem::WrongExtension)
        );
    }

    #[test]
    fn extension_without_name_is_invalid() {
        assert_eq!(
            Addon64Url::parse("https://a.github.io/renodx/.addon64"),
            Err(LinkProblem::WrongExtension)
        );
    }

    #[test]
    fn query_is_invalid() {
        assert_eq!(
            Addon64Url::parse(
                "https://github.com/a/renodx/releases/download/snapshot/x.addon64?raw=true"
            ),
            Err(LinkProblem::HasQueryOrFragment)
        );
    }

    #[test]
    fn trailing_slash_is_invalid() {
        assert_eq!(
            Addon64Url::parse("https://a.github.io/renodx/renodx-x.addon64/"),
            Err(LinkProblem::WrongExtension)
        );
    }

    #[test]
    fn display_with_padding_returns_the_padded_url() {
        let value =
            Addon64Url::parse("https://fornkatt.github.io/renodx/renodx-control.addon64").unwrap();

        assert_eq!(
            format!("{value:*<59}"),
            "https://fornkatt.github.io/renodx/renodx-control.addon64***"
        );
    }

    #[test]
    fn file_name_comparison_keys_differing_only_in_case_are_equal() {
        let first =
            Addon64Url::parse("https://fornkatt.github.io/renodx/renodx-control.addon64").unwrap();
        let second =
            Addon64Url::parse("https://fornkatt.github.io/renodx/RenoDX-Control.addon64").unwrap();

        assert_eq!(
            first.file_name_comparison_key(),
            second.file_name_comparison_key()
        );
    }
}

mod addon_32_url {
    use super::*;

    #[test]
    fn github_release_addon_is_valid() {
        let url = Addon32Url::parse(
            "https://github.com/fornkatt/renodx/releases/download/snapshot/renodx-control.addon32",
        )
        .unwrap();

        assert_eq!(url.file_name(), "renodx-control.addon32");
    }

    #[test]
    fn addon_64_file_is_invalid() {
        assert_eq!(
            Addon32Url::parse("https://a.github.io/renodx/renodx-x.addon64"),
            Err(LinkProblem::WrongExtension)
        );
    }

    #[test]
    fn display_with_padding_returns_the_padded_url() {
        let value =
            Addon32Url::parse("https://fornkatt.github.io/renodx/renodx-control.addon32").unwrap();

        assert_eq!(
            format!("{value:*<59}"),
            "https://fornkatt.github.io/renodx/renodx-control.addon32***"
        );
    }

    #[test]
    fn file_name_comparison_keys_differing_only_in_case_are_equal() {
        let first =
            Addon32Url::parse("https://fornkatt.github.io/renodx/renodx-control.addon32").unwrap();
        let second =
            Addon32Url::parse("https://fornkatt.github.io/renodx/RenoDX-Control.addon32").unwrap();

        assert_eq!(
            first.file_name_comparison_key(),
            second.file_name_comparison_key()
        );
    }
}
