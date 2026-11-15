// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use domain::renodx::GameMod;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameModSummary {
    pub name: String,
}

impl GameModSummary {
    pub(super) fn from_game_mod(game_mod: &GameMod) -> GameModSummary {
        GameModSummary {
            name: game_mod.name().as_str().to_string(),
        }
    }
}
