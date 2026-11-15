// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use application::Failure;
use application::renodx::game_mods::{GameModSummary, GameModsEditor, LoadGameMods};
use generated::{GameModRow, MainWindow};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

#[allow(
    trivial_numeric_casts,
    missing_debug_implementations,
    clippy::semicolon_outside_block,
    clippy::clone_on_ref_ptr,
    reason = "Slint-generated code"
)]
mod generated {
    slint::include_modules!();
}

pub fn run<F: LoadGameMods>(game_mods_file: &F) -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    match GameModsEditor::open(game_mods_file) {
        Ok(editor) => show_game_mods(&main_window, &editor),
        Err(failure) => show_failure(&main_window, &failure),
    }
    main_window.run()
}

fn show_game_mods(main_window: &MainWindow, editor: &GameModsEditor) {
    let mut rows = Vec::new();

    for summary in editor.summaries() {
        rows.push(game_mod_row(&summary));
    }

    main_window.set_game_mods(ModelRc::new(VecModel::from(rows)));
    main_window.set_notices(text_lines(&editor.notices()));
}

fn show_failure(main_window: &MainWindow, failure: &Failure) {
    main_window.set_failure_summary(SharedString::from(failure.summary.as_str()));
    main_window.set_failure_details(text_lines(&failure.details));
}

fn game_mod_row(summary: &GameModSummary) -> GameModRow {
    GameModRow {
        name: SharedString::from(summary.name.as_str()),
    }
}

fn text_lines(lines: &[String]) -> ModelRc<SharedString> {
    let mut shared_lines = Vec::new();

    for line in lines {
        shared_lines.push(SharedString::from(line.as_str()));
    }
    ModelRc::new(VecModel::from(shared_lines))
}
