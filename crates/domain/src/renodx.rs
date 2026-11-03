// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

mod addon_url;
mod game_mod;
mod game_mods;
mod status;

pub use addon_url::{ADDON_32_EXTENSION, ADDON_64_EXTENSION, Addon32Url, Addon64Url};
pub use game_mod::{GameMod, GameModParts, GameModProblem, ModLinks};
pub use game_mods::{AddonDrift, DuplicateName, GameMods, GameModsProblem};
pub use status::ModStatus;

pub const OFFICIAL_REPOSITORY_OWNER: &str = "clshortfuse";
pub const OFFICIAL_REPOSITORY_NAME: &str = "renodx";
