// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(test)]
mod tests;

use super::messages;
use super::{LoadGameMods, LoadedGameMods, StoreGameMods};
use crate::{Failure, FileStamp};
use domain::id::ModId;
use domain::renodx::{GameMod, GameMods, GameModsProblem};
use domain::text::GameName;
use std::collections::BTreeMap;
use std::error::Error;

#[derive(Debug)]
pub struct GameModsEditor {
    game_mods: GameMods,
    stamp: FileStamp,
}

impl GameModsEditor {
    pub fn open<F: LoadGameMods>(file: &F) -> Result<GameModsEditor, Failure> {
        let LoadedGameMods { entries, stamp } = match file.load() {
            Ok(loaded) => loaded,
            Err(error) => {
                tracing::warn!(
                    error = %error,
                    source = ?error.source(),
                    "opening the game mods file failed"
                );
                return Err(messages::load_failure(&error));
            }
        };
        let (game_mods, duplicate_ids) = game_mods_from_entries(entries);

        if !duplicate_ids.is_empty() {
            tracing::warn!(
                duplicate_id_count = duplicate_ids.len(),
                "refused a game mods file with duplicate ids"
            );
            return Err(messages::duplicate_ids_failure(&duplicate_ids));
        }
        tracing::info!(entries = game_mods.len(), "opened the game mods file");

        Ok(GameModsEditor { game_mods, stamp })
    }

    pub fn save<F: StoreGameMods>(&mut self, file: &F) -> Result<(), Failure> {
        match file.store(&self.game_mods, &self.stamp) {
            Ok(stamp) => {
                self.stamp = stamp;

                tracing::info!(entries = self.game_mods.len(), "saved the game mods file");

                Ok(())
            }
            Err(error) => {
                tracing::warn!(
                    error = %error,
                    source = ?error.source(),
                    "saving the game mods file failed"
                );
                Err(messages::store_failure(&error))
            }
        }
    }

    pub fn notices(&self) -> Vec<String> {
        let mut notices = Vec::new();

        for duplicate_name in self.game_mods.duplicate_names() {
            notices.push(messages::duplicate_name_notice(&duplicate_name));
        }
        for drift in self.game_mods.addon_drift() {
            notices.push(messages::addon_drift_notice(&drift));
        }
        notices
    }
}

pub(super) struct DuplicateId {
    pub(super) id: ModId,
    pub(super) entries: Vec<DuplicateEntry>,
}

pub(super) struct DuplicateEntry {
    pub(super) position: usize,
    pub(super) name: GameName,
}

fn game_mods_from_entries(entries: Vec<GameMod>) -> (GameMods, Vec<DuplicateId>) {
    let mut game_mods = GameMods::new();
    let mut first_positions: BTreeMap<ModId, usize> = BTreeMap::new();
    let mut rejected: BTreeMap<ModId, Vec<DuplicateEntry>> = BTreeMap::new();

    for (index, game_mod) in entries.into_iter().enumerate() {
        let position = index + 1;
        let id = game_mod.id().clone();
        let name = game_mod.name().clone();

        match game_mods.add(game_mod) {
            Ok(()) => {
                first_positions.insert(id, position);
            }
            Err(GameModsProblem::DuplicateId) => {
                rejected
                    .entry(id)
                    .or_default()
                    .push(DuplicateEntry { position, name });
            }
            Err(GameModsProblem::UnknownId) => {
                unreachable!("GameMods::add never reports an unknown id")
            }
        }
    }

    let mut duplicate_ids = Vec::new();

    for (id, later_entries) in rejected {
        duplicate_ids.push(duplicate_id(
            &game_mods,
            &first_positions,
            id,
            later_entries,
        ));
    }
    (game_mods, duplicate_ids)
}

fn duplicate_id(
    game_mods: &GameMods,
    first_positions: &BTreeMap<ModId, usize>,
    id: ModId,
    later_entries: Vec<DuplicateEntry>,
) -> DuplicateId {
    let first = game_mods
        .get(&id)
        .expect("a rejected id belongs to an entry that was added before it");
    let mut entries = vec![DuplicateEntry {
        position: first_positions[&id],
        name: first.name().clone(),
    }];

    entries.extend(later_entries);

    DuplicateId { id, entries }
}
