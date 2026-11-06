// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use super::*;

mod parse_game_mods {
    use super::*;
    use domain::link::LinkProblem;
    use domain::text::TextProblem;

    #[test]
    fn entries_are_returned_in_file_order() {
        let text = document(&[
            entry("zelda", "Zelda", "working", NEXUS_ONLY_LINKS),
            entry("alan-wake", "Alan Wake", "working", NEXUS_ONLY_LINKS),
        ]);
        let game_mods = parse_game_mods(&text).unwrap();

        assert_eq!(ids_of(&game_mods), vec!["zelda", "alan-wake"]);
    }

    #[test]
    fn status_texts_map_to_mod_statuses() {
        let text = document(&[
            entry("control", "Control", "working", NEXUS_ONLY_LINKS),
            entry("alan-wake", "Alan Wake", "wip", NEXUS_ONLY_LINKS),
        ]);
        let game_mods = parse_game_mods(&text).unwrap();
        let statuses: Vec<ModStatus> = game_mods.iter().map(GameMod::status).collect();

        assert_eq!(
            statuses,
            vec![ModStatus::Working, ModStatus::WorkInProgress]
        );
    }

    #[test]
    fn missing_optional_keys_read_as_none() {
        let text = r#"{"schemaVersion": 1, "games": [{
            "id": "control", "name": "Control", "maintainer": "John Rust", "status": "working",
            "links": {"nexusMods": "https://www.nexusmods.com/control/mods/1"}
        }]}"#;
        let game_mods = parse_game_mods(text).unwrap();

        assert_eq!(game_mods[0].notes(), None);
        assert_eq!(game_mods[0].links().snapshot_64, None);
    }

    #[test]
    fn notes_and_every_link_are_read() {
        let links = r#"{
            "snapshot64": "https://fornkatt.github.io/renodx/renodx-control.addon64",
            "snapshot32": "https://fornkatt.github.io/renodx/renodx-control.addon32",
            "nexusMods": "https://www.nexusmods.com/control/mods/1",
            "gameBanana": "https://gamebanana.com/mods/680225",
            "discord": "https://discord.gg/renodx",
            "discussion": "https://github.com/clshortfuse/renodx/discussions/490"
        }"#;
        let text = document(&[entry("control", "Control", "working", links)
            .replace(r#""notes": null"#, r#""notes": "Lacks testing.""#)]);
        let game_mods = parse_game_mods(&text).unwrap();
        let game_mod = &game_mods[0];
        let links = game_mod.links();

        assert_eq!(game_mod.notes().map(Notes::as_str), Some("Lacks testing."));
        assert_eq!(
            links.snapshot_64.as_ref().map(Addon64Url::as_str),
            Some("https://fornkatt.github.io/renodx/renodx-control.addon64")
        );
        assert_eq!(
            links.snapshot_32.as_ref().map(Addon32Url::as_str),
            Some("https://fornkatt.github.io/renodx/renodx-control.addon32")
        );
        assert_eq!(
            links.nexus_mods.as_ref().map(NexusModsUrl::as_str),
            Some("https://www.nexusmods.com/control/mods/1")
        );
        assert_eq!(
            links.game_banana.as_ref().map(GameBananaUrl::as_str),
            Some("https://gamebanana.com/mods/680225")
        );
        assert_eq!(
            links.discord.as_ref().map(DiscordUrl::as_str),
            Some("https://discord.gg/renodx")
        );
        assert_eq!(
            links.discussion.as_ref().map(DiscussionUrl::as_str),
            Some("https://github.com/clshortfuse/renodx/discussions/490")
        );
    }

    #[test]
    fn leading_byte_order_mark_is_accepted() {
        let text = format!(
            "\u{FEFF}{}",
            document(&[entry("control", "Control", "working", NEXUS_ONLY_LINKS)])
        );
        let result = parse_game_mods(&text);

        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn empty_games_list_returns_no_entries() {
        let game_mods = parse_game_mods(&document(&[])).unwrap();

        assert_eq!(game_mods, vec![]);
    }

    #[test]
    fn unknown_key_in_an_entry_is_an_unexpected_shape() {
        let text = document(&[entry("control", "Control", "working", NEXUS_ONLY_LINKS)
            .replace(r#""status""#, r#""rating": 5, "status""#)]);
        let result = parse_game_mods(&text);
        let Err(LoadGameModsError::UnexpectedShape { parser_message, .. }) = result else {
            panic!("expected UnexpectedShape, got {result:?}");
        };

        assert!(
            parser_message.starts_with("unknown field `rating`"),
            "{parser_message}"
        );
    }

    #[test]
    fn unknown_top_level_key_is_an_unexpected_shape() {
        let text = r#"{"schemaVersion": 1, "games": [], "addedLater": true}"#;
        let result = parse_game_mods(text);

        assert!(
            matches!(result, Err(LoadGameModsError::UnexpectedShape { .. })),
            "{result:?}"
        );
    }

    #[test]
    fn unknown_key_in_links_is_an_unexpected_shape() {
        let links = NEXUS_ONLY_LINKS.replace(r#""discord""#, r#""steam": null, "discord""#);
        let text = document(&[entry("control", "Control", "working", &links)]);
        let result = parse_game_mods(&text);

        assert!(
            matches!(result, Err(LoadGameModsError::UnexpectedShape { .. })),
            "{result:?}"
        );
    }

    #[test]
    fn newer_schema_version_is_unsupported() {
        let text = r#"{"schemaVersion": 2, "games": [], "addedLater": true}"#;
        let result = parse_game_mods(text);

        assert!(
            matches!(
                result,
                Err(LoadGameModsError::UnsupportedSchemaVersion {
                    found: 2,
                    supported: 1
                })
            ),
            "{result:?}"
        );
    }

    #[test]
    fn invalid_entries_are_all_reported() {
        let text = document(&[
            entry("control", "Control", "working", NEXUS_ONLY_LINKS),
            entry(
                "alan-wake",
                "Alan Wake",
                "working",
                &NEXUS_ONLY_LINKS.replace("https://", "http://"),
            ),
            entry("quantum-break", "", "working", NEXUS_ONLY_LINKS),
        ]);
        let result = parse_game_mods(&text);
        let Err(LoadGameModsError::InvalidEntries(problems)) = result else {
            panic!("expected InvalidEntries, got {result:?}");
        };

        assert_eq!(
            problems,
            vec![
                EntryProblem {
                    position: 2,
                    id_text: "alan-wake".to_string(),
                    name_text: "Alan Wake".to_string(),
                    problem: EntryFieldProblem::NexusMods(LinkProblem::NotHttps),
                },
                EntryProblem {
                    position: 3,
                    id_text: "quantum-break".to_string(),
                    name_text: String::new(),
                    problem: EntryFieldProblem::Name(TextProblem::Empty),
                },
            ]
        );
    }
}

mod load {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn existing_file_is_parsed() {
        let repository = TempDir::new().unwrap();
        let folder = repository.path().join("db").join("renodx");

        fs::create_dir_all(&folder).unwrap();

        let text = document(&[entry("control", "Control", "working", NEXUS_ONLY_LINKS)]);

        fs::write(folder.join("game-mods.json"), text).unwrap();

        let game_mods = GameModsJsonFile::in_repository(repository.path())
            .load()
            .unwrap();

        assert_eq!(ids_of(&game_mods), vec!["control"]);
    }

    #[test]
    fn missing_file_is_file_not_found() {
        let repository = TempDir::new().unwrap();
        let result = GameModsJsonFile::in_repository(repository.path()).load();
        let Err(LoadGameModsError::FileNotFound { path }) = result else {
            panic!("expected FileNotFound, got {result:?}");
        };

        assert!(path.ends_with("db/renodx/game-mods.json"), "{path:?}");
    }
}

const NEXUS_ONLY_LINKS: &str = r#"{"snapshot64": null, "snapshot32": null,
"nexusMods": "https://www.nexusmods.com/control/mods/1", "gameBanana": null, "discord": null,
 "discussion": null}"#;

fn document(entries: &[String]) -> String {
    format!(
        r#"{{"schemaVersion": 1, "games": [{}]}}"#,
        entries.join(", ")
    )
}

fn entry(id: &str, name: &str, status: &str, links: &str) -> String {
    format!(
        r#"{{"id": "{id}", "name": "{name}", "maintainer": "John Rust", "status": "{status}",
         "notes": null, "links": {links}}}"#
    )
}

fn ids_of(game_mods: &[GameMod]) -> Vec<&str> {
    game_mods
        .iter()
        .map(|game_mod| game_mod.id().as_str())
        .collect()
}
