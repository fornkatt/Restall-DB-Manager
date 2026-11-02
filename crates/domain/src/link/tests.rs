// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use super::*;

mod parse_https_url {
    use super::*;

    #[test]
    fn uppercase_host_returns_lowercase_canonical_url() {
        let url = parse_https_url("https://WWW.NexusMods.com/x").unwrap();

        assert_eq!(url.as_str(), "https://www.nexusmods.com/x");
    }

    #[test]
    fn relative_url_is_invalid() {
        assert_eq!(
            parse_https_url("github.com/x"),
            Err(LinkProblem::NotAValidUrl)
        );
    }

    #[test]
    fn http_is_invalid() {
        assert_eq!(
            parse_https_url("http://github.com/x"),
            Err(LinkProblem::NotHttps)
        );
    }

    #[test]
    fn missing_host_typo_is_invalid() {
        assert_eq!(parse_https_url("https:///x"), Err(LinkProblem::InvalidHost));
    }

    #[test]
    fn ip_address_is_invalid() {
        assert_eq!(
            parse_https_url("https://127.0.0.1/x"),
            Err(LinkProblem::InvalidHost)
        );
    }

    #[test]
    fn trailing_dot_host_is_invalid() {
        assert_eq!(
            parse_https_url("https://github.com./x"),
            Err(LinkProblem::InvalidHost)
        );
    }

    #[test]
    fn credentials_are_invalid() {
        assert_eq!(
            parse_https_url("https://user:secret@github.com/x"),
            Err(LinkProblem::HasCredentialsOrPort)
        );
    }

    #[test]
    fn custom_port_is_invalid() {
        assert_eq!(
            parse_https_url("https://github.com:8443/x"),
            Err(LinkProblem::HasCredentialsOrPort)
        );
    }

    #[test]
    fn empty_label_host_is_invalid() {
        assert_eq!(
            parse_https_url("https://.com/x"),
            Err(LinkProblem::InvalidHost)
        );
    }
}

mod nexus_mods_url {
    use super::*;

    #[test]
    fn mod_page_with_query_is_valid() {
        let url = NexusModsUrl::parse("https://www.nexusmods.com/avowed/mods/101?tab=description")
            .unwrap();

        assert_eq!(
            url.as_str(),
            "https://www.nexusmods.com/avowed/mods/101?tab=description"
        );
    }

    #[test]
    fn other_host_is_invalid() {
        assert_eq!(
            NexusModsUrl::parse("https://gamebanana.com/mods/680225"),
            Err(LinkProblem::WrongHost)
        );
    }

    #[test]
    fn display_with_padding_returns_the_padded_url() {
        let value = NexusModsUrl::parse("https://www.nexusmods.com/control/mods/1").unwrap();

        assert_eq!(
            format!("{value:*<43}"),
            "https://www.nexusmods.com/control/mods/1***"
        );
    }
}

mod game_banana_url {
    use super::*;

    #[test]
    fn mod_page_is_valid() {
        let result = GameBananaUrl::parse("https://gamebanana.com/mods/680225");

        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn other_host_is_invalid() {
        assert_eq!(
            GameBananaUrl::parse("https://www.nexusmods.com/avowed/mods/101"),
            Err(LinkProblem::WrongHost)
        );
    }

    #[test]
    fn display_with_padding_returns_the_padded_url() {
        let value = GameBananaUrl::parse("https://gamebanana.com/mods/680225").unwrap();

        assert_eq!(
            format!("{value:*<37}"),
            "https://gamebanana.com/mods/680225***"
        );
    }
}

mod discord_url {
    use super::*;

    #[test]
    fn invite_short_link_is_valid() {
        let result = DiscordUrl::parse("https://discord.gg/renodx");

        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn other_host_is_invalid() {
        assert_eq!(
            DiscordUrl::parse("https://www.nexusmods.com/avowed/mod/101"),
            Err(LinkProblem::WrongHost)
        );
    }

    #[test]
    fn display_with_padding_returns_the_padded_url() {
        let value = DiscordUrl::parse("https://discord.gg/renodx").unwrap();

        assert_eq!(format!("{value:*<28}"), "https://discord.gg/renodx***");
    }
}

mod discussion_url {
    use super::*;

    #[test]
    fn discussion_with_trailing_slash_is_valid() {
        let result = DiscussionUrl::parse("https://github.com/clshortfuse/renodx/discussions/490/");

        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn issue_link_is_invalid() {
        assert_eq!(
            DiscussionUrl::parse("https://github.com/clshortfuse/renodx/issues/490"),
            Err(LinkProblem::WrongPath)
        );
    }

    #[test]
    fn fork_discussion_is_not_in_official_repository() {
        let url =
            DiscussionUrl::parse("https://github.com/fornkatt/renodx/discussions/12").unwrap();

        assert!(!url.is_in_repository("clshortfuse", "renodx"));
    }

    #[test]
    fn mixed_case_owner_is_in_repository() {
        let url =
            DiscussionUrl::parse("https://github.com/ClShortFuse/RenoDX/discussions/490").unwrap();

        assert!(url.is_in_repository("clshortfuse", "renodx"));
    }

    #[test]
    fn display_with_padding_returns_the_padded_url() {
        let value =
            DiscussionUrl::parse("https://github.com/clshortfuse/renodx/discussions/490").unwrap();

        assert_eq!(
            format!("{value:*<56}"),
            "https://github.com/clshortfuse/renodx/discussions/490***"
        );
    }
}
