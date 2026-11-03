// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use super::*;

use crate::link::NexusModsUrl;
use crate::renodx::{Addon32Url, Addon64Url, GameModParts, ModLinks, ModStatus};
use crate::text::Maintainer;

mod new {
    use super::*;

    #[test]
    fn new_game_mods_are_empty() {
        let game_mods = GameMods::new();

        assert!(game_mods.is_empty());
        assert_eq!(game_mods.len(), 0);
    }
}

mod is_empty {
    use super::*;

    #[test]
    fn game_mods_with_an_entry_are_not_empty() {
        let game_mods = game_mods_with(vec![game_mod_with_nexus_link("control", "Control")]);

        assert!(!game_mods.is_empty());
    }
}

mod add {
    use super::*;

    #[test]
    fn new_id_is_added() {
        let mut game_mods = GameMods::new();
        let result = game_mods.add(game_mod_with_nexus_link(
            "control-resonant",
            "Control RESONANT",
        ));

        assert!(result.is_ok(), "{result:?}");
        assert_eq!(
            name_of(&game_mods, "control-resonant"),
            Some("Control RESONANT")
        );
    }

    #[test]
    fn duplicate_id_is_rejected() {
        let mut game_mods = game_mods_with(vec![game_mod_with_nexus_link("control", "Control")]);
        let result = game_mods.add(game_mod_with_nexus_link(
            "control",
            "Control Ultimate Edition",
        ));

        assert_eq!(result, Err(GameModsProblem::DuplicateId));
        assert_eq!(name_of(&game_mods, "control"), Some("Control"));
    }
}

mod replace {
    use super::*;

    #[test]
    fn existing_entry_is_replaced() {
        let mut game_mods = game_mods_with(vec![game_mod_with_nexus_link("control", "Control")]);
        let result = game_mods.replace(
            &id("control"),
            game_mod_with_nexus_link("control", "Control Ultimate Edition"),
        );

        assert!(result.is_ok(), "{result:?}");
        assert_eq!(
            name_of(&game_mods, "control"),
            Some("Control Ultimate Edition")
        );
    }

    #[test]
    fn unknown_id_is_rejected() {
        let mut game_mods = game_mods_with(vec![game_mod_with_nexus_link("control", "Control")]);
        let result = game_mods.replace(
            &id("alan-wake"),
            game_mod_with_nexus_link("alan-wake", "Alan Wake"),
        );

        assert_eq!(result, Err(GameModsProblem::UnknownId));
        assert_eq!(game_mods.len(), 1);
    }

    #[test]
    fn new_id_taken_by_another_entry_is_rejected() {
        let mut game_mods = game_mods_with(vec![
            game_mod_with_nexus_link("alan-wake", "Alan Wake"),
            game_mod_with_nexus_link("control", "Control"),
        ]);
        let result = game_mods.replace(
            &id("control"),
            game_mod_with_nexus_link("alan-wake", "Control"),
        );

        assert_eq!(result, Err(GameModsProblem::DuplicateId));
        assert_eq!(name_of(&game_mods, "alan-wake"), Some("Alan Wake"));
        assert_eq!(name_of(&game_mods, "control"), Some("Control"));
    }

    #[test]
    fn changing_to_a_free_id_moves_the_entry() {
        let mut game_mods = game_mods_with(vec![game_mod_with_nexus_link("contrl", "Control")]);
        let result = game_mods.replace(
            &id("contrl"),
            game_mod_with_nexus_link("control", "Control"),
        );

        assert!(result.is_ok(), "{result:?}");
        assert_eq!(name_of(&game_mods, "contrl"), None);
        assert_eq!(name_of(&game_mods, "control"), Some("Control"));
    }
}

mod remove {
    use super::*;

    #[test]
    fn existing_entry_is_returned() {
        let mut game_mods = game_mods_with(vec![game_mod_with_nexus_link("control", "Control")]);
        let removed = game_mods.remove(&id("control"));

        assert_eq!(
            removed.map(|game_mod| game_mod.id().clone()),
            Some(id("control"))
        );
        assert!(game_mods.is_empty());
    }

    #[test]
    fn unknown_id_returns_none() {
        let mut game_mods = game_mods_with(vec![game_mod_with_nexus_link("control", "Control")]);
        let removed = game_mods.remove(&id("alan-wake"));

        assert_eq!(removed, None);
        assert_eq!(game_mods.len(), 1);
    }
}

mod iter {
    use super::*;

    #[test]
    fn returns_entries_in_id_order() {
        let game_mods = game_mods_with(vec![
            game_mod_with_nexus_link("control-resonant", "Control RESONANT"),
            game_mod_with_nexus_link("alan-wake", "Alan Wake"),
            game_mod_with_nexus_link("control", "Control"),
        ]);
        let ids: Vec<&str> = game_mods
            .iter()
            .map(|game_mod| game_mod.id().as_str())
            .collect();

        assert_eq!(ids, vec!["alan-wake", "control", "control-resonant"]);
    }
}

mod find_by_name {
    use super::*;

    #[test]
    fn name_differing_only_in_case_is_found() {
        let game_mods = game_mods_with(vec![
            game_mod_with_nexus_link("alan-wake", "Alan Wake"),
            game_mod_with_nexus_link("control", "Control"),
        ]);
        let found = game_mods.find_by_name(&name("CONTROL"));

        assert_eq!(
            found.map(|game_mod| game_mod.id().as_str()),
            Some("control")
        );
    }

    #[test]
    fn unknown_name_returns_none() {
        let game_mods = game_mods_with(vec![game_mod_with_nexus_link("control", "Control")]);
        let found = game_mods.find_by_name(&name("Alan Wake"));

        assert_eq!(found.map(|game_mod| game_mod.id().as_str()), None);
    }
}

mod duplicate_names {
    use super::*;

    #[test]
    fn names_used_by_several_entries_are_reported() {
        let game_mods = game_mods_with(vec![
            game_mod_with_nexus_link("alan-wake", "Alan Wake"),
            game_mod_with_nexus_link("control", "Control"),
            game_mod_with_nexus_link("control-resonant", "CONTROL"),
        ]);
        let duplicates = game_mods.duplicate_names();

        assert_eq!(
            duplicates,
            vec![DuplicateName {
                name: name("Control"),
                ids: vec![id("control"), id("control-resonant")]
            }]
        );
    }

    #[test]
    fn unique_names_report_nothing() {
        let game_mods = game_mods_with(vec![
            game_mod_with_nexus_link("alan-wake", "Alan Wake"),
            game_mod_with_nexus_link("control", "Control"),
        ]);

        assert_eq!(game_mods.duplicate_names(), vec![]);
    }
}

mod addon_drift {
    use super::*;

    #[test]
    fn shared_file_with_different_urls_is_reported() {
        let game_mods = game_mods_with(vec![
            game_mod_with_addon_64("elden-ring", "ELDEN RING", FROMSOFT_URL),
            game_mod_with_addon_64("sekiro", "Sekiro", FROMSOFT_URL),
            game_mod_with_addon_64("trails-1", "Trails 1", FALCOM_URL),
            game_mod_with_addon_64("trails-2", "Trails 2", FALCOM_MOVED_URL),
        ]);
        let drift = game_mods.addon_drift();

        assert_eq!(
            drift,
            vec![AddonDrift {
                file_name: "renodx-falcom.addon64".to_string(),
                ids: vec![id("trails-1"), id("trails-2")],
                distinct_urls: vec![FALCOM_URL.to_string(), FALCOM_MOVED_URL.to_string()],
            }]
        );
    }

    #[test]
    fn file_names_differing_only_in_case_are_the_same_file() {
        let game_mods = game_mods_with(vec![
            game_mod_with_addon_64("trails-1", "Trails 1", FALCOM_URL),
            game_mod_with_addon_64("trails-2", "Trails 2", FALCOM_UPPERCASE_URL),
        ]);
        let drift = game_mods.addon_drift();

        assert_eq!(
            drift
                .iter()
                .map(|group| group.ids.clone())
                .collect::<Vec<_>>(),
            vec![vec![id("trails-1"), id("trails-2")]]
        );
    }

    #[test]
    fn shared_32_bit_file_with_different_urls_is_reported() {
        let game_mods = game_mods_with(vec![
            game_mod_with_addon_32("dishonored", "Dishonored", DISHONORED_32_URL),
            game_mod_with_addon_32("dishonored-2", "Dishonored 2", DISHONORED_32_MOVED_URL),
        ]);
        let drift = game_mods.addon_drift();

        assert_eq!(
            drift
                .iter()
                .map(|group| group.ids.clone())
                .collect::<Vec<_>>(),
            vec![vec![id("dishonored"), id("dishonored-2")]]
        );
    }
}

const FROMSOFT_URL: &str = "https://fornkatt.github.io/renodx/renodx-fromsoft.addon64";
const FALCOM_URL: &str = "https://fornkatt.github.io/renodx/renodx-falcom.addon64";
const FALCOM_UPPERCASE_URL: &str = "https://fornkatt.github.io/renodx/RenoDX-Falcom.addon64";
const FALCOM_MOVED_URL: &str =
    "https://github.com/fornkatt/renodx/releases/download/snapshot/renodx-falcom.addon64";
const DISHONORED_32_URL: &str = "https://fornkatt.github.io/renodx/renodx-dishonored.addon32";
const DISHONORED_32_MOVED_URL: &str =
    "https://github.com/fornkatt/renodx/releases/download/snapshot/renodx-dishonored.addon32";

fn game_mods_with(entries: Vec<GameMod>) -> GameMods {
    let mut game_mods = GameMods::new();

    for game_mod in entries {
        game_mods.add(game_mod).unwrap();
    }
    game_mods
}
fn name_of<'a>(game_mods: &'a GameMods, id_text: &str) -> Option<&'a str> {
    game_mods
        .get(&id(id_text))
        .map(|game_mod| game_mod.name().as_str())
}
fn game_mod_with_nexus_link(id_text: &str, name_text: &str) -> GameMod {
    let links = ModLinks {
        nexus_mods: Some(NexusModsUrl::parse("https://www.nexusmods.com/control/mods/1").unwrap()),
        ..links_without_any()
    };
    game_mod_with_links(id_text, name_text, links)
}
fn game_mod_with_addon_64(id_text: &str, name_text: &str, url: &str) -> GameMod {
    let links = ModLinks {
        snapshot_64: Some(Addon64Url::parse(url).unwrap()),
        ..links_without_any()
    };
    game_mod_with_links(id_text, name_text, links)
}
fn game_mod_with_addon_32(id_text: &str, name_text: &str, url: &str) -> GameMod {
    let links = ModLinks {
        snapshot_32: Some(Addon32Url::parse(url).unwrap()),
        ..links_without_any()
    };
    game_mod_with_links(id_text, name_text, links)
}
fn game_mod_with_links(id_text: &str, name_text: &str, links: ModLinks) -> GameMod {
    GameMod::new(GameModParts {
        id: id(id_text),
        name: name(name_text),
        maintainer: Maintainer::parse("John Rust").unwrap(),
        status: ModStatus::Working,
        notes: None,
        links,
    })
    .unwrap()
}
fn links_without_any() -> ModLinks {
    ModLinks {
        snapshot_64: None,
        snapshot_32: None,
        nexus_mods: None,
        game_banana: None,
        discord: None,
        discussion: None,
    }
}
fn id(text: &str) -> ModId {
    ModId::parse(text).unwrap()
}
fn name(text: &str) -> GameName {
    GameName::parse(text).unwrap()
}
