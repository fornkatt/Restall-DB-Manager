// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use slint::ComponentHandle;

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

pub fn run() -> Result<(), slint::PlatformError> {
    let main_window = generated::MainWindow::new()?;

    main_window.run()
}
