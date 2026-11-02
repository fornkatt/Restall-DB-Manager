// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(test)]
mod tests;

use crate::id::ModId;
use crate::link::{DiscordUrl, DiscussionUrl, GameBananaUrl, NexusModsUrl};
use crate::renodx::{Addon32Url, Addon64Url, ModStatus};
use crate::text::{GameName, Maintainer, Notes};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModLinks {
    pub snapshot_64: Option<Addon64Url>,
    pub snapshot_32: Option<Addon32Url>,
    pub nexus_mods: Option<NexusModsUrl>,
    pub game_banana: Option<GameBananaUrl>,
    pub discord: Option<DiscordUrl>,
    pub discussion: Option<DiscussionUrl>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameModParts {
    pub id: ModId,
    pub name: GameName,
    pub maintainer: Maintainer,
    pub status: ModStatus,
    pub notes: Option<Notes>,
    pub links: ModLinks,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameMod {
    parts: GameModParts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameModProblem {
    NoSourceLink,
}

impl ModLinks {
    /// Whether any link other than the discussion link is set.
    pub fn has_source_link(&self) -> bool {
        self.snapshot_64.is_some()
            || self.snapshot_32.is_some()
            || self.nexus_mods.is_some()
            || self.game_banana.is_some()
            || self.discord.is_some()
    }
}

impl GameMod {
    pub fn new(parts: GameModParts) -> Result<GameMod, GameModProblem> {
        if !parts.links.has_source_link() {
            return Err(GameModProblem::NoSourceLink);
        }
        Ok(GameMod { parts })
    }
    pub fn into_parts(self) -> GameModParts {
        self.parts
    }
    pub fn id(&self) -> &ModId {
        &self.parts.id
    }
    pub fn name(&self) -> &GameName {
        &self.parts.name
    }
    pub fn maintainer(&self) -> &Maintainer {
        &self.parts.maintainer
    }
    pub fn status(&self) -> ModStatus {
        self.parts.status
    }
    pub fn notes(&self) -> Option<&Notes> {
        self.parts.notes.as_ref()
    }
    pub fn links(&self) -> &ModLinks {
        &self.parts.links
    }
}
