// SPDX-FileCopyrightText: 2026 Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    ui::run()?;

    Ok(())
}
