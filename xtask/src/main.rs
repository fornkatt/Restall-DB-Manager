// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

mod spdx_headers;

use std::process::ExitCode;

fn main() -> ExitCode {
    let argument_strings: Vec<String> = std::env::args().skip(1).collect();
    let arguments: Vec<&str> = argument_strings
        .iter()
        .map(|argument| argument.as_str())
        .collect();

    match arguments.as_slice() {
        ["spdx-headers"] => spdx_headers::add_missing(),
        ["spdx-headers", "--check"] => spdx_headers::check(),
        _ => print_usage(),
    }
}

fn print_usage() -> ExitCode {
    eprintln!("Usage: cargo xtask spdx-headers [--check]");

    ExitCode::FAILURE
}
