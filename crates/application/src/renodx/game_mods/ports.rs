// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(test)]
mod tests;

use crate::FileStamp;
use domain::id::ModIdProblem;
use domain::link::LinkProblem;
use domain::renodx::{GameMod, GameModProblem, GameMods};
use domain::text::TextProblem;
use std::error::Error;
use std::path::PathBuf;
use std::{fmt, io};

pub trait LoadGameMods {
    /// Returns the entries in the order the file lists them, with a stamp of the file as read.
    fn load(&self) -> Result<LoadedGameMods, LoadGameModsError>;
}

#[derive(Debug)]
pub struct LoadedGameMods {
    pub entries: Vec<GameMod>,
    pub stamp: FileStamp,
}

#[derive(Debug)]
pub enum LoadGameModsError {
    FileNotFound {
        path: PathBuf,
    },
    NotUtf8 {
        path: PathBuf,
    },
    Unreadable {
        path: PathBuf,
        source: io::Error,
    },
    NotJson {
        line: usize,
        column: usize,
        parser_message: String,
    },
    UnexpectedShape {
        line: usize,
        column: usize,
        parser_message: String,
    },
    UnsupportedSchemaVersion {
        found: u64,
        supported: u64,
    },
    InvalidEntries(Vec<EntryProblem>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryProblem {
    /// Counted from 1, in the order the file lists the entries.
    pub position: usize,
    pub id_text: String,
    pub name_text: String,
    pub problem: EntryFieldProblem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryFieldProblem {
    Id(ModIdProblem),
    Name(TextProblem),
    Maintainer(TextProblem),
    Notes(TextProblem),
    Snapshot64(LinkProblem),
    Snapshot32(LinkProblem),
    NexusMods(LinkProblem),
    GameBanana(LinkProblem),
    Discord(LinkProblem),
    Discussion(LinkProblem),
    Entry(GameModProblem),
}

impl fmt::Display for LoadGameModsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadGameModsError::FileNotFound { path } => {
                write!(f, "game mods file not found: {}", path.display())
            }
            LoadGameModsError::NotUtf8 { path } => {
                write!(f, "game mods file is not UTF-8: {}", path.display())
            }
            LoadGameModsError::Unreadable { path, .. } => {
                write!(f, "game mods file could not be read: {}", path.display())
            }
            LoadGameModsError::NotJson {
                line,
                column,
                parser_message,
            } => write!(
                f,
                "game mods file is not valid JSON at line {line} column {column}: {parser_message}"
            ),
            LoadGameModsError::UnexpectedShape {
                line,
                column,
                parser_message,
            } => write!(
                f,
                "game mods file has unexpected shape at line {line} column {column}: {parser_message}"
            ),
            LoadGameModsError::UnsupportedSchemaVersion { found, supported } => write!(
                f,
                "game mods file has schema version {found}, only version {supported} is supported"
            ),
            LoadGameModsError::InvalidEntries(problems) => {
                write!(f, "game mods file has {} invalid entries", problems.len())
            }
        }
    }
}

impl Error for LoadGameModsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoadGameModsError::Unreadable { source, .. } => Some(source),
            LoadGameModsError::FileNotFound { .. }
            | LoadGameModsError::NotUtf8 { .. }
            | LoadGameModsError::NotJson { .. }
            | LoadGameModsError::UnexpectedShape { .. }
            | LoadGameModsError::UnsupportedSchemaVersion { .. }
            | LoadGameModsError::InvalidEntries(_) => None,
        }
    }
}

pub trait StoreGameMods {
    /// Writes the entries in id order.
    ///
    /// Refuses when the file no longer matches `expected`, and returns the stamp of what it wrote.
    fn store(
        &self,
        game_mods: &GameMods,
        expected: &FileStamp,
    ) -> Result<FileStamp, StoreGameModsError>;
}

#[derive(Debug)]
pub enum StoreGameModsError {
    ChangedOnDisk { path: PathBuf },
    RemovedFromDisk { path: PathBuf },
    Unreadable { path: PathBuf, source: io::Error },
    Unwritable { path: PathBuf, source: io::Error },
}

impl fmt::Display for StoreGameModsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreGameModsError::ChangedOnDisk { path } => {
                write!(f, "game mods file changed on disk: {}", path.display())
            }
            StoreGameModsError::RemovedFromDisk { path } => {
                write!(
                    f,
                    "game mods file was removed from disk: {}",
                    path.display()
                )
            }
            StoreGameModsError::Unreadable { path, .. } => {
                write!(f, "game mods file could not be read: {}", path.display())
            }
            StoreGameModsError::Unwritable { path, .. } => {
                write!(f, "game mods file could not be written: {}", path.display())
            }
        }
    }
}

impl Error for StoreGameModsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            StoreGameModsError::Unreadable { source, .. }
            | StoreGameModsError::Unwritable { source, .. } => Some(source),
            StoreGameModsError::ChangedOnDisk { .. }
            | StoreGameModsError::RemovedFromDisk { .. } => None,
        }
    }
}
