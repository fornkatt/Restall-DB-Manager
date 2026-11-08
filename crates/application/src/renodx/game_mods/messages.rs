// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use super::editor::{DuplicateEntry, DuplicateId};
use super::{EntryFieldProblem, EntryProblem, LoadGameModsError, StoreGameModsError};
use crate::Failure;
use domain::id::{ModId, ModIdProblem};
use domain::link::LinkProblem;
use domain::renodx::{
    ADDON_32_EXTENSION, ADDON_64_EXTENSION, AddonDrift, DuplicateName, GameModProblem,
};
use domain::text::TextProblem;
use std::path::Path;

const SEE_THE_LOG: &str = "See the log for details.";
const EDITS_ARE_KEPT: &str = "Your edits are still here.";

pub(super) fn load_failure(error: &LoadGameModsError) -> Failure {
    match error {
        LoadGameModsError::FileNotFound { path } => failure(
            "The game mods file was not found.",
            vec![
                file_detail(path),
                "Check that the repository folder is a clone of the mod database.".to_string(),
            ],
        ),
        LoadGameModsError::NotUtf8 { path } => failure(
            "The game mods file is not valid UTF-8.",
            vec![file_detail(path)],
        ),
        LoadGameModsError::Unreadable { path, .. } => failure(
            "The game mods file could not be read.",
            vec![file_detail(path), SEE_THE_LOG.to_string()],
        ),
        LoadGameModsError::NotJson {
            line,
            column,
            parser_message,
        } => failure(
            "The game mods file is not valid JSON.",
            vec![position_detail(*line, *column, parser_message)],
        ),
        LoadGameModsError::UnexpectedShape {
            line,
            column,
            parser_message,
        } => failure(
            "The game mods file doesn't have the expected structure.",
            vec![position_detail(*line, *column, parser_message)],
        ),
        LoadGameModsError::UnsupportedSchemaVersion { found, supported } => failure(
            &format!("The game mods file uses schema version {found}."),
            vec![
                format!("This editor supports schema version {supported}."),
                "Use an editor version that supports this file.".to_string(),
            ],
        ),
        LoadGameModsError::InvalidEntries(problems) => {
            let mut details = Vec::new();

            for problem in problems {
                details.push(entry_problem_detail(problem));
            }

            failure(&invalid_entries_summary(problems.len()), details)
        }
    }
}

pub(super) fn duplicate_ids_failure(duplicate_ids: &[DuplicateId]) -> Failure {
    let mut details = Vec::new();

    for duplicate_id in duplicate_ids {
        details.push(duplicate_id_detail(duplicate_id));
    }

    failure(
        "The game mods file uses the same id for more than one entry.",
        details,
    )
}

pub(super) fn store_failure(error: &StoreGameModsError) -> Failure {
    match error {
        StoreGameModsError::ChangedOnDisk { path } => failure(
            "Not saved. The game mods file changed outside the editor since the last open or save.",
            vec![
                EDITS_ARE_KEPT.to_string(),
                "To load the other changes, open the file again. That discards your unsaved edits."
                    .to_string(),
                file_detail(path),
            ],
        ),
        StoreGameModsError::RemovedFromDisk { path } => failure(
            "Not saved. The game mods file was removed since the last open or save.",
            vec![
                EDITS_ARE_KEPT.to_string(),
                "Restore the file (for example with git), then save again.".to_string(),
                file_detail(path),
            ],
        ),
        StoreGameModsError::Unreadable { path, .. } => failure(
            "Not saved. The game mods file could not be checked for outside changes.",
            vec![
                EDITS_ARE_KEPT.to_string(),
                file_detail(path),
                SEE_THE_LOG.to_string(),
            ],
        ),
        StoreGameModsError::Unwritable { path, .. } => failure(
            "Not saved. The game mods file could not be written.",
            vec![
                EDITS_ARE_KEPT.to_string(),
                file_detail(path),
                SEE_THE_LOG.to_string(),
            ],
        ),
    }
}

pub(super) fn duplicate_name_notice(duplicate: &DuplicateName) -> String {
    format!(
        "\"{}\" is the name of {} entries: {}.",
        duplicate.name,
        duplicate.ids.len(),
        comma_separated(&duplicate.ids)
    )
}

pub(super) fn addon_drift_notice(drift: &AddonDrift) -> String {
    format!(
        "The addon file {} is linked with {} different URLs by the entries {}.",
        drift.file_name,
        drift.distinct_urls.len(),
        comma_separated(&drift.ids)
    )
}

fn failure(summary: &str, details: Vec<String>) -> Failure {
    Failure {
        summary: summary.to_string(),
        details,
    }
}

fn file_detail(path: &Path) -> String {
    format!("File: {}", path.display())
}

fn position_detail(line: usize, column: usize, parser_message: &str) -> String {
    format!("Line {line}, column {column}: {parser_message}.")
}

fn invalid_entries_summary(count: usize) -> String {
    if count == 1 {
        "The game mods file has 1 invalid entry.".to_string()
    } else {
        format!("The game mods file has {count} invalid entries.")
    }
}

fn entry_problem_detail(problem: &EntryProblem) -> String {
    format!(
        "Entry {} ({}, \"{}\"): {}.",
        problem.position,
        problem.id_text,
        problem.name_text,
        field_problem_text(problem.problem)
    )
}

fn duplicate_id_detail(duplicate_id: &DuplicateId) -> String {
    let mut entries = Vec::new();

    for entry in &duplicate_id.entries {
        entries.push(duplicate_entry_text(entry));
    }

    format!(
        "The id \"{}\" is used by {}.",
        duplicate_id.id,
        joined_with_and(&entries)
    )
}

fn duplicate_entry_text(entry: &DuplicateEntry) -> String {
    format!("entry {} (\"{}\")", entry.position, entry.name)
}

fn field_problem_text(problem: EntryFieldProblem) -> String {
    match problem {
        EntryFieldProblem::Id(problem) => format!("the id {}", mod_id_problem_phrase(problem)),
        EntryFieldProblem::Name(problem) => format!("the name {}", text_problem_phrase(problem)),
        EntryFieldProblem::Maintainer(problem) => {
            format!("the maintainer {}", text_problem_phrase(problem))
        }
        EntryFieldProblem::Notes(problem) => {
            format!("the notes text {}", text_problem_phrase(problem))
        }
        EntryFieldProblem::Snapshot64(LinkProblem::WrongExtension) => {
            format!("the 64-bit snapshot link doesn't end with {ADDON_64_EXTENSION}")
        }
        EntryFieldProblem::Snapshot64(problem) => {
            format!("the 64-bit snapshot link {}", link_problem_phrase(problem))
        }
        EntryFieldProblem::Snapshot32(LinkProblem::WrongExtension) => {
            format!("the 32-bit snapshot link doesn't end with {ADDON_32_EXTENSION}")
        }
        EntryFieldProblem::Snapshot32(problem) => {
            format!("the 32-bit snapshot link {}", link_problem_phrase(problem))
        }
        EntryFieldProblem::NexusMods(problem) => {
            format!("the Nexus Mods link {}", link_problem_phrase(problem))
        }
        EntryFieldProblem::GameBanana(problem) => {
            format!("the GameBanana link {}", link_problem_phrase(problem))
        }
        EntryFieldProblem::Discord(problem) => {
            format!("the Discord link {}", link_problem_phrase(problem))
        }
        EntryFieldProblem::Discussion(problem) => {
            format!(
                "the GitHub discussion link {}",
                link_problem_phrase(problem)
            )
        }
        EntryFieldProblem::Entry(problem) => game_mod_problem_text(problem).to_string(),
    }
}

fn text_problem_phrase(problem: TextProblem) -> &'static str {
    match problem {
        TextProblem::Empty => "is empty or only whitespace",
        TextProblem::SurroundingWhitespace => "starts or ends with whitespace",
        TextProblem::ControlCharacter => "contains a control character",
        TextProblem::InvisibleCharacter => "contains an invisible character",
    }
}

fn mod_id_problem_phrase(problem: ModIdProblem) -> String {
    match problem {
        ModIdProblem::Empty => "is empty".to_string(),
        ModIdProblem::TooLong { max } => format!("is longer than {max} characters"),
        ModIdProblem::InvalidCharacter => {
            "may only contain lowercase letters a-z, digits and hyphens".to_string()
        }
        ModIdProblem::StartsOrEndsWithHyphen => "starts or ends with a hyphen".to_string(),
        ModIdProblem::DoubleHyphen => "contains two hyphens in a row".to_string(),
    }
}

fn link_problem_phrase(problem: LinkProblem) -> &'static str {
    match problem {
        LinkProblem::NotAValidUrl => "is not a valid web address",
        LinkProblem::NotHttps => "is not HTTPS",
        LinkProblem::HasCredentialsOrPort => "contains a user name, a password or a port",
        LinkProblem::InvalidHost => "has an invalid host name",
        LinkProblem::HasDoubleSlashInPath => "contains // in its path",
        LinkProblem::WrongHost => "points to the wrong website",
        LinkProblem::WrongPath => "points to an unexpected page on its website",
        LinkProblem::HasQueryOrFragment => "contains a ? or # part",
        LinkProblem::WrongExtension => "doesn't end with the expected file extension",
    }
}

fn game_mod_problem_text(problem: GameModProblem) -> &'static str {
    match problem {
        GameModProblem::NoSourceLink => {
            "it has no source link (a snapshot, Nexus Mods, GameBanana or Discord link)"
        }
    }
}

fn comma_separated(ids: &[ModId]) -> String {
    let mut texts = Vec::new();

    for id in ids {
        texts.push(id.as_str());
    }
    texts.join(", ")
}

fn joined_with_and(parts: &[String]) -> String {
    match parts {
        [] => String::new(),
        [only] => only.clone(),
        [all_but_last @ .., last] => format!("{} and {last}", all_but_last.join(", ")),
    }
}
