// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use slint::ComponentHandle;

mod generated {
    slint::include_modules!();
}

pub fn run() -> Result<(), slint::PlatformError> {
    let main_window = generated::MainWindow::new()?;

    main_window.run()
}
