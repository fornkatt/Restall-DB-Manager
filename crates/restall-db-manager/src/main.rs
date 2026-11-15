// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use infrastructure::renodx::GameModsJsonFile;
use std::env;
use std::path::PathBuf;

const CURRENT_FOLDER: &str = ".";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repository_folder = repository_folder_from_arguments();
    let game_mods_file = GameModsJsonFile::in_repository(&repository_folder);

    ui::run(&game_mods_file)?;

    Ok(())
}

fn repository_folder_from_arguments() -> PathBuf {
    match env::args_os().nth(1) {
        Some(argument) => PathBuf::from(argument),
        None => PathBuf::from(CURRENT_FOLDER),
    }
}
