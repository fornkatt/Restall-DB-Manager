// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

mod record;
#[cfg(test)]
mod tests;

use application::renodx::game_mods::{
    EntryFieldProblem, EntryProblem, LoadGameMods, LoadGameModsError,
};
use domain::id::ModId;
use domain::link::{DiscordUrl, DiscussionUrl, GameBananaUrl, NexusModsUrl};
use domain::renodx::{Addon32Url, Addon64Url, GameMod, GameModParts, ModLinks, ModStatus};
use domain::text::{GameName, Maintainer, Notes};
use record::{GameModRecord, LinksRecord, StatusRecord, GameModsDocument, SchemaVersionProbe};
use serde_json::error::Category;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Where the file lives inside a clone of the DB repository.
///
/// Since Restall downloads the same path from the repository, moving the file breaks it.
const GAME_MODS_FILE_PATH: [&str; 3] = ["db", "renodx", "game-mods.json"];
const SUPPORTED_SCHEMA_VERSION: u64 = 1;
const BYTE_ORDER_MARK: char = '\u{FEFF}';

#[derive(Debug)]
pub struct GameModsJsonFile {
    path: PathBuf,
}

impl GameModsJsonFile {
    pub fn in_repository(repository_root: &Path) -> GameModsJsonFile {
        let mut path = repository_root.to_path_buf();

        for component in GAME_MODS_FILE_PATH {
            path.push(component);
        }

        GameModsJsonFile { path }
    }
}

impl LoadGameMods for GameModsJsonFile {
    fn load(&self) -> Result<Vec<GameMod>, LoadGameModsError> {
        let text = fs::read_to_string(&self.path).map_err(|error| file_error(&self.path, error))?;

        parse_game_mods(&text)
    }
}

fn file_error(path: &Path, error: io::Error) -> LoadGameModsError {
    match error.kind() {
        io::ErrorKind::NotFound => LoadGameModsError::FileNotFound {
            path: path.to_path_buf(),
        },
        io::ErrorKind::InvalidData => LoadGameModsError::NotUtf8 {
            path: path.to_path_buf(),
        },
        _ => LoadGameModsError::Unreadable {
            path: path.to_path_buf(),
            source: error,
        },
    }
}

fn parse_game_mods(text: &str) -> Result<Vec<GameMod>, LoadGameModsError> {
    let text = text.strip_prefix(BYTE_ORDER_MARK).unwrap_or(text);
    let probe: SchemaVersionProbe =
        serde_json::from_str(text).map_err(|error| json_error(&error))?;

    if probe.schema_version != SUPPORTED_SCHEMA_VERSION {
        return Err(LoadGameModsError::UnsupportedSchemaVersion {
            found: probe.schema_version,
            supported: SUPPORTED_SCHEMA_VERSION,
        });
    }
    let document: GameModsDocument =
        serde_json::from_str(text).map_err(|error| json_error(&error))?;

    game_mods_from_records(&document.games)
}

fn json_error(error: &serde_json::Error) -> LoadGameModsError {
    let line = error.line();
    let column = error.column();
    let parser_message = parser_message_without_position(error);

    match error.classify() {
        Category::Syntax | Category::Eof => LoadGameModsError::NotJson {
            line,
            column,
            parser_message,
        },
        Category::Data => LoadGameModsError::UnexpectedShape {
            line,
            column,
            parser_message,
        },
        Category::Io => unreachable!("parsing text from memory does no I/O"),
    }
}

fn parser_message_without_position(error: &serde_json::Error) -> String {
    let message = error.to_string();
    let position = format!(" at line {} column {}", error.line(), error.column());

    match message.strip_suffix(&position) {
        Some(without_position) => without_position.to_string(),
        None => message,
    }
}

fn game_mods_from_records(records: &[GameModRecord]) -> Result<Vec<GameMod>, LoadGameModsError> {
    let mut game_mods = Vec::with_capacity(records.len());
    let mut problems = Vec::new();

    for (index, record) in records.iter().enumerate() {
        match game_mod_from_record(record) {
            Ok(game_mod) => game_mods.push(game_mod),
            Err(problem) => problems.push(EntryProblem {
                position: index + 1,
                id_text: record.id.clone(),
                name_text: record.name.clone(),
                problem,
            }),
        }
    }
    if problems.is_empty() {
        Ok(game_mods)
    } else {
        Err(LoadGameModsError::InvalidEntries(problems))
    }
}

fn game_mod_from_record(record: &GameModRecord) -> Result<GameMod, EntryFieldProblem> {
    let parts = GameModParts {
        id: ModId::parse(&record.id).map_err(EntryFieldProblem::Id)?,
        name: GameName::parse(&record.name).map_err(EntryFieldProblem::Name)?,
        maintainer: Maintainer::parse(&record.maintainer).map_err(EntryFieldProblem::Maintainer)?,
        status: mod_status_from_record(record.status),
        notes: notes_from_record(record.notes.as_deref())?,
        links: links_from_record(&record.links)?,
    };

    GameMod::new(parts).map_err(EntryFieldProblem::Entry)
}

fn mod_status_from_record(status: StatusRecord) -> ModStatus {
    match status {
        StatusRecord::Working => ModStatus::Working,
        StatusRecord::WorkInProgress => ModStatus::WorkInProgress,
    }
}

fn notes_from_record(text: Option<&str>) -> Result<Option<Notes>, EntryFieldProblem> {
    match text {
        Some(text) => Ok(Some(Notes::parse(text).map_err(EntryFieldProblem::Notes)?)),
        None => Ok(None),
    }
}

fn links_from_record(record: &LinksRecord) -> Result<ModLinks, EntryFieldProblem> {
    let snapshot_64 = match record.snapshot_64.as_deref() {
        Some(text) => Some(Addon64Url::parse(text).map_err(EntryFieldProblem::Snapshot64)?),
        None => None,
    };
    let snapshot_32 = match record.snapshot_32.as_deref() {
        Some(text) => Some(Addon32Url::parse(text).map_err(EntryFieldProblem::Snapshot32)?),
        None => None,
    };
    let nexus_mods = match record.nexus_mods.as_deref() {
        Some(text) => Some(NexusModsUrl::parse(text).map_err(EntryFieldProblem::NexusMods)?),
        None => None,
    };
    let game_banana = match record.game_banana.as_deref() {
        Some(text) => Some(GameBananaUrl::parse(text).map_err(EntryFieldProblem::GameBanana)?),
        None => None,
    };
    let discord = match record.discord.as_deref() {
        Some(text) => Some(DiscordUrl::parse(text).map_err(EntryFieldProblem::Discord)?),
        None => None,
    };
    let discussion = match record.discussion.as_deref() {
        Some(text) => Some(DiscussionUrl::parse(text).map_err(EntryFieldProblem::Discussion)?),
        None => None,
    };
    Ok(ModLinks {
        snapshot_64,
        snapshot_32,
        nexus_mods,
        game_banana,
        discord,
        discussion,
    })
}
