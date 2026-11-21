// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use application::Failure;
use application::renodx::game_mods::GameModSummary;
use generated::{GameModRow, MainWindow};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use std::fmt;
use worker::{OpenedGameMods, ResultSink, WorkRequests, WorkResult};

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

pub struct UserInterface {
    main_window: MainWindow,
}

pub struct UserInterfaceSink {
    main_window: slint::Weak<MainWindow>,
}

impl UserInterface {
    pub fn new() -> Result<UserInterface, slint::PlatformError> {
        let main_window = MainWindow::new()?;

        Ok(UserInterface { main_window })
    }

    pub fn result_sink(&self) -> UserInterfaceSink {
        UserInterfaceSink {
            main_window: self.main_window.as_weak(),
        }
    }

    pub fn run(self, requests: WorkRequests) -> Result<(), slint::PlatformError> {
        let main_window = self.main_window.as_weak();

        request_game_mods(&self.main_window, &requests);

        self.main_window.on_reload_requested(move || {
            if let Some(main_window) = main_window.upgrade() {
                request_game_mods(&main_window, &requests);
            }
        });

        self.main_window.run()
    }
}

impl fmt::Debug for UserInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserInterface").finish_non_exhaustive()
    }
}

impl ResultSink for UserInterfaceSink {
    fn deliver(&self, result: WorkResult) {
        let delivery = self
            .main_window
            .upgrade_in_event_loop(move |main_window| show_result(&main_window, &result));

        if let Err(error) = delivery {
            tracing::debug!(
                error = %error,
                "a work result arrived after the user interface ended"
            );
        }
    }
}

impl fmt::Debug for UserInterfaceSink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserInterfaceSink").finish_non_exhaustive()
    }
}

fn request_game_mods(main_window: &MainWindow, requests: &WorkRequests) {
    if let Err(failure) = requests.open_game_mods() {
        show_failure(main_window, &failure);
    }
}

fn show_result(main_window: &MainWindow, result: &WorkResult) {
    match result {
        WorkResult::OpenGameMods(Ok(opened)) => show_opened_game_mods(main_window, opened),
        WorkResult::OpenGameMods(Err(failure)) => show_failure(main_window, failure),
    }
}

fn show_opened_game_mods(main_window: &MainWindow, opened: &OpenedGameMods) {
    let mut rows = Vec::new();

    for summary in &opened.summaries {
        rows.push(game_mod_row(summary));
    }

    main_window.set_game_mods(ModelRc::new(VecModel::from(rows)));
    main_window.set_notices(text_lines(&opened.notices));
    main_window.set_failure_summary(SharedString::new());
    main_window.set_failure_details(ModelRc::default());
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
