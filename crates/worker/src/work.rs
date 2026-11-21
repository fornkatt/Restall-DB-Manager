// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use application::Failure;
use application::renodx::game_mods::GameModSummary;

#[derive(Debug)]
pub(crate) enum WorkRequest {
    OpenGameMods,
    Stop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkResult {
    OpenGameMods(Result<OpenedGameMods, Failure>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenedGameMods {
    pub summaries: Vec<GameModSummary>,
    pub notices: Vec<String>,
}

pub trait ResultSink: Send + 'static {
    fn deliver(&self, result: WorkResult);
}
