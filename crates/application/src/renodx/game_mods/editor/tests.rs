// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use super::*;
use crate::renodx::game_mods::{LoadGameModsError, StoreGameModsError};
use domain::link::NexusModsUrl;
use domain::renodx::{GameModParts, ModLinks, ModStatus};
use domain::text::Maintainer;
use std::cell::RefCell;
use std::path::PathBuf;

mod open {
    use super::*;

    #[test]
    fn duplicate_ids_are_refused_naming_both_entries() {
        let file = FakeGameModsFile::containing(vec![
            game_mod("control", "Control"),
            game_mod("alan-wake", "Alan Wake"),
            game_mod("control", "Control RESONANT"),
        ]);
        let failure = GameModsEditor::open(&file).unwrap_err();

        assert_eq!(
            failure.details,
            vec![
                "The id \"control\" is used by entry 1 (\"Control\") and entry 3 (\"Control RESONANT\")."
                    .to_string()
            ]
        );
    }

    #[test]
    fn every_entry_is_kept() {
        let file = FakeGameModsFile::containing(vec![
            game_mod("control", "Control"),
            game_mod("alan-wake", "Alan Wake"),
        ]);
        let mut editor = GameModsEditor::open(&file).unwrap();

        editor.save(&file).unwrap();

        assert_eq!(file.stored_ids(), vec![vec!["alan-wake", "control"]]);
    }
}

mod save {
    use super::*;

    #[test]
    fn first_save_uses_the_loaded_stamp() {
        let file = FakeGameModsFile::containing(vec![game_mod("control", "Control")]);
        let mut editor = GameModsEditor::open(&file).unwrap();
        let result = editor.save(&file);

        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn second_save_succeeds() {
        let file = FakeGameModsFile::containing(vec![game_mod("control", "Control")]);
        let mut editor = GameModsEditor::open(&file).unwrap();

        editor.save(&file).unwrap();

        let result = editor.save(&file);

        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn changed_file_is_refused_and_edits_are_kept() {
        let file = FakeGameModsFile::containing(vec![
            game_mod("control", "Control"),
            game_mod("alan-wake", "Alan Wake"),
        ]);
        let mut editor = GameModsEditor::open(&file).unwrap();

        file.change_outside_the_editor();

        let refused = editor.save(&file);

        file.undo_outside_change();

        let saved = editor.save(&file);

        assert!(refused.is_err(), "{refused:?}");
        assert!(saved.is_ok(), "{saved:?}");
        assert_eq!(file.stored_ids(), vec![vec!["alan-wake", "control"]]);
    }
}

const ORIGINAL_DISK_CONTENT: &[u8] = b"the file as it was opened";
const OUTSIDE_CHANGE: &[u8] = b"the file after someone edited it by hand";

struct FakeGameModsFile {
    entries: Vec<GameMod>,
    disk: RefCell<Vec<u8>>,
    stored_ids: RefCell<Vec<Vec<String>>>,
}

impl FakeGameModsFile {
    fn containing(entries: Vec<GameMod>) -> FakeGameModsFile {
        FakeGameModsFile {
            entries,
            disk: RefCell::new(ORIGINAL_DISK_CONTENT.to_vec()),
            stored_ids: RefCell::new(Vec::new()),
        }
    }

    fn change_outside_the_editor(&self) {
        *self.disk.borrow_mut() = OUTSIDE_CHANGE.to_vec();
    }

    fn undo_outside_change(&self) {
        *self.disk.borrow_mut() = ORIGINAL_DISK_CONTENT.to_vec();
    }

    fn stored_ids(&self) -> Vec<Vec<String>> {
        self.stored_ids.borrow().clone()
    }
}

impl LoadGameMods for FakeGameModsFile {
    fn load(&self) -> Result<LoadedGameMods, LoadGameModsError> {
        Ok(LoadedGameMods {
            entries: self.entries.clone(),
            stamp: FileStamp::from_content(self.disk.borrow().clone()),
        })
    }
}

impl StoreGameMods for FakeGameModsFile {
    fn store(
        &self,
        game_mods: &GameMods,
        expected: &FileStamp,
    ) -> Result<FileStamp, StoreGameModsError> {
        if !expected.matches(&self.disk.borrow()) {
            return Err(StoreGameModsError::ChangedOnDisk {
                path: PathBuf::from("game-mods.json"),
            });
        }

        let mut ids = Vec::new();

        for game_mod in game_mods {
            ids.push(game_mod.id().as_str().to_string());
        }

        let written = ids.join(",").into_bytes();

        *self.disk.borrow_mut() = written.clone();
        self.stored_ids.borrow_mut().push(ids);

        Ok(FileStamp::from_content(written))
    }
}

fn game_mod(id: &str, name: &str) -> GameMod {
    GameMod::new(GameModParts {
        id: ModId::parse(id).unwrap(),
        name: GameName::parse(name).unwrap(),
        maintainer: Maintainer::parse("John Rust").unwrap(),
        status: ModStatus::Working,
        notes: None,
        links: ModLinks {
            snapshot_64: None,
            snapshot_32: None,
            nexus_mods: Some(
                NexusModsUrl::parse("https://www.nexusmods.com/control/mods/1").unwrap(),
            ),
            game_banana: None,
            discord: None,
            discussion: None,
        },
    })
    .unwrap()
}
