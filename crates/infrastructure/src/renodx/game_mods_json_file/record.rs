// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct SchemaVersionProbe {
    pub(super) schema_version: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct GameModsDocument {
    #[expect(dead_code, reason = "read once the adapter writes the file")]
    pub(super) schema_version: u64,
    pub(super) games: Vec<GameModRecord>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct GameModRecord {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) maintainer: String,
    pub(super) status: StatusRecord,
    pub(super) notes: Option<String>,
    pub(super) links: LinksRecord,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(super) enum StatusRecord {
    #[serde(rename = "working")]
    Working,
    #[serde(rename = "wip")]
    WorkInProgress,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct LinksRecord {
    pub(super) snapshot_64: Option<String>,
    pub(super) snapshot_32: Option<String>,
    pub(super) nexus_mods: Option<String>,
    pub(super) game_banana: Option<String>,
    pub(super) discord: Option<String>,
    pub(super) discussion: Option<String>,
}
