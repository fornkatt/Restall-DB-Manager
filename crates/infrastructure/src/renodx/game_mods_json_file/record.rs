// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct SchemaVersionProbe {
    pub(super) schema_version: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct GameModsDocument {
    pub(super) schema_version: u64,
    pub(super) games: Vec<GameModRecord>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct GameModRecord {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) maintainer: String,
    pub(super) status: StatusRecord,
    pub(super) required_graphics_api: Option<GraphicsApiRecord>,
    pub(super) notes: Option<String>,
    pub(super) links: LinksRecord,
    #[serde(default)]
    pub(super) depends: Vec<ComponentNeedRecord>,
    #[serde(default)]
    pub(super) recommends: Vec<ComponentNeedRecord>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub(super) enum StatusRecord {
    #[serde(rename = "working")]
    Working,
    #[serde(rename = "wip")]
    WorkInProgress,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub(super) enum GraphicsApiRecord {
    #[serde(rename = "d3d9")]
    D3d9,
    #[serde(rename = "d3d10")]
    D3d10,
    #[serde(rename = "d3d11")]
    D3d11,
    #[serde(rename = "d3d12")]
    D3d12,
    #[serde(rename = "openGl")]
    OpenGl,
    #[serde(rename = "vulkan")]
    Vulkan,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct LinksRecord {
    pub(super) snapshot_64: Option<String>,
    pub(super) snapshot_32: Option<String>,
    pub(super) nexus_mods: Option<String>,
    pub(super) game_banana: Option<String>,
    pub(super) discord: Option<String>,
    pub(super) discussion: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct ComponentNeedRecord {
    pub(super) component: ComponentRecord,
    pub(super) notes: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub(super) enum ComponentRecord {
    #[serde(rename = "reFramework")]
    ReFramework,
    #[serde(rename = "dxvkSource1ReShadeProxy")]
    DxvkSource1ReShadeProxy,
}
