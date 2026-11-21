// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use infrastructure::renodx::GameModsJsonFile;
use std::env;
use std::path::PathBuf;
use ui::UserInterface;
use worker::Worker;

const CURRENT_FOLDER: &str = ".";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repository_folder = repository_folder_from_arguments();
    let game_mods_file = GameModsJsonFile::in_repository(&repository_folder);
    let user_interface = UserInterface::new()?;
    let Worker { requests, thread } = Worker::start(game_mods_file, user_interface.result_sink())?;
    let ui_outcome = user_interface.run(requests);

    thread.finish().expect("the worker thread should not panic");

    ui_outcome?;

    Ok(())
}

fn repository_folder_from_arguments() -> PathBuf {
    match env::args_os().nth(1) {
        Some(argument) => PathBuf::from(argument),
        None => PathBuf::from(CURRENT_FOLDER),
    }
}
