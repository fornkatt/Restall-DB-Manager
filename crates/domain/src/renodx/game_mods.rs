// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(test)]
mod tests;

use crate::id::ModId;
use crate::renodx::GameMod;
use crate::text::GameName;
use std::collections::{BTreeMap, btree_map};

#[derive(Debug, Default)]
pub struct GameMods {
    entries: BTreeMap<ModId, GameMod>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameModsProblem {
    DuplicateId,
    UnknownId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateName {
    pub name: GameName,
    pub ids: Vec<ModId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddonDrift {
    pub file_name: String,
    pub ids: Vec<ModId>,
    pub distinct_urls: Vec<String>,
}

impl GameMods {
    pub fn new() -> GameMods {
        GameMods::default()
    }
    pub fn add(&mut self, game_mod: GameMod) -> Result<(), GameModsProblem> {
        if self.entries.contains_key(game_mod.id()) {
            return Err(GameModsProblem::DuplicateId);
        }
        self.entries.insert(game_mod.id().clone(), game_mod);
        Ok(())
    }
    pub fn replace(&mut self, old_id: &ModId, game_mod: GameMod) -> Result<(), GameModsProblem> {
        if !self.entries.contains_key(old_id) {
            return Err(GameModsProblem::UnknownId);
        }
        if game_mod.id() != old_id && self.entries.contains_key(game_mod.id()) {
            return Err(GameModsProblem::DuplicateId);
        }
        self.entries.remove(old_id);
        self.entries.insert(game_mod.id().clone(), game_mod);

        Ok(())
    }
    pub fn remove(&mut self, id: &ModId) -> Option<GameMod> {
        self.entries.remove(id)
    }
    pub fn get(&self, id: &ModId) -> Option<&GameMod> {
        self.entries.get(id)
    }
    pub fn find_by_name(&self, name: &GameName) -> Option<&GameMod> {
        let key = name.comparison_key();

        self.entries
            .values()
            .find(|game_mod| game_mod.name().comparison_key() == key)
    }
    pub fn iter(&self) -> btree_map::Values<'_, ModId, GameMod> {
        self.entries.values()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn duplicate_names(&self) -> Vec<DuplicateName> {
        let mut groups: BTreeMap<String, DuplicateName> = BTreeMap::new();

        for game_mod in self.entries.values() {
            let group = groups
                .entry(game_mod.name().comparison_key())
                .or_insert_with(|| DuplicateName {
                    name: game_mod.name().clone(),
                    ids: Vec::new(),
                });
            group.ids.push(game_mod.id().clone());
        }
        groups
            .into_values()
            .filter(|group| group.ids.len() > 1)
            .collect()
    }
    pub fn addon_drift(&self) -> Vec<AddonDrift> {
        let mut groups: BTreeMap<String, AddonDrift> = BTreeMap::new();

        for game_mod in self.entries.values() {
            let links = game_mod.links();

            if let Some(url) = &links.snapshot_64 {
                let key = url.file_name_comparison_key();

                add_to_addon_group(
                    &mut groups,
                    key,
                    url.file_name(),
                    url.as_str(),
                    game_mod.id(),
                );
            }
            if let Some(url) = &links.snapshot_32 {
                let key = url.file_name_comparison_key();

                add_to_addon_group(
                    &mut groups,
                    key,
                    url.file_name(),
                    url.as_str(),
                    game_mod.id(),
                );
            }
        }
        groups
            .into_values()
            .filter(|group| group.distinct_urls.len() > 1)
            .collect()
    }
}

impl<'a> IntoIterator for &'a GameMods {
    type Item = &'a GameMod;
    type IntoIter = btree_map::Values<'a, ModId, GameMod>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn add_to_addon_group(
    groups: &mut BTreeMap<String, AddonDrift>,
    file_name_key: String,
    file_name: &str,
    url: &str,
    id: &ModId,
) {
    let group = groups.entry(file_name_key).or_insert_with(|| AddonDrift {
        file_name: file_name.to_string(),
        ids: Vec::new(),
        distinct_urls: Vec::new(),
    });
    group.ids.push(id.clone());

    if !group.distinct_urls.iter().any(|known_url| known_url == url) {
        group.distinct_urls.push(url.to_string());
    }
}
