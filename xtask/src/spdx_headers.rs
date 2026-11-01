// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const SPDX_PREFIX: &str = "// SPDX-";
const COPYRIGHT_PREFIX: &str = "// SPDX-FileCopyrightText: ";
const LICENSE_PREFIX: &str = "// SPDX-License-Identifier: ";
const LICENSE_ID: &str = "GPL-3.0-only";
const DEFAULT_COPYRIGHT_HOLDER: &str = "Johan Lager";
const BYTE_ORDER_MARK: char = '\u{feff}';
const SOURCE_FILE_EXTENSIONS: &[&str] = &["rs", "slint"];
const BUILD_OUTPUT_DIRECTORY: &str = "target";

#[derive(Debug, PartialEq)]
enum HeaderStatus {
    Valid,
    Missing,
    Invalid(HeaderProblem),
}
#[derive(Debug, PartialEq)]
enum HeaderProblem {
    StartsWithByteOrderMark,
    NotAtTop,
    NoCopyrightLine,
    NoLicenseLine,
    WrongLicense,
    NoBlankLineAfter,
}

pub fn add_missing() -> ExitCode {
    let Some(source_files) = collect_workspace_source_files() else {
        return ExitCode::FAILURE;
    };
    let mut added_count = 0;
    let mut problem_count = 0;

    for file in &source_files {
        let Some(content) = read_source_file(file) else {
            problem_count += 1;
            continue;
        };

        match classify_header(&content) {
            HeaderStatus::Valid => {}
            HeaderStatus::Missing => match fs::write(file, prepend_default_header(&content)) {
                Ok(()) => {
                    println!(
                        "added header: {}",
                        relative_to_workspace_root(file).display()
                    );
                    added_count += 1;
                }
                Err(error) => {
                    eprintln!(
                        "could not write {}: {error}",
                        relative_to_workspace_root(file).display()
                    );
                    problem_count += 1;
                }
            },
            HeaderStatus::Invalid(problem) => {
                print_header_problem(file, &problem);
                problem_count += 1;
            }
        }
    }
    println!("Added {added_count} header(s).");

    match problem_count {
        0 => ExitCode::SUCCESS,
        _ => {
            println!("{problem_count} source file(s) need fixing by hand.");
            ExitCode::FAILURE
        }
    }
}

pub fn check() -> ExitCode {
    let Some(source_files) = collect_workspace_source_files() else {
        return ExitCode::FAILURE;
    };
    let mut missing_count = 0;
    let mut problem_count = 0;

    for file in &source_files {
        let Some(content) = read_source_file(file) else {
            problem_count += 1;
            continue;
        };
        match classify_header(&content) {
            HeaderStatus::Valid => {}
            HeaderStatus::Missing => {
                println!(
                    "missing header: {}",
                    relative_to_workspace_root(file).display()
                );
                missing_count += 1;
                problem_count += 1;
            }
            HeaderStatus::Invalid(problem) => {
                print_header_problem(file, &problem);
                problem_count += 1;
            }
        }
    }
    match problem_count {
        0 => {
            println!(
                "All {} source files have valid SPDX headers.",
                source_files.len()
            );
            ExitCode::SUCCESS
        }
        _ => {
            println!("{problem_count} source file(s) need attention.");
            if missing_count > 0 {
                println!("Run `cargo xtask spdx-headers` to add the missing headers.");
            }
            ExitCode::FAILURE
        }
    }
}

fn classify_header(content: &str) -> HeaderStatus {
    if content.starts_with(BYTE_ORDER_MARK) {
        return HeaderStatus::Invalid(HeaderProblem::StartsWithByteOrderMark);
    }
    let lines: Vec<&str> = content.lines().collect();

    if !lines.iter().any(|line| line.starts_with(SPDX_PREFIX)) {
        return HeaderStatus::Missing;
    }
    match find_header_problem(&lines) {
        Some(problem) => HeaderStatus::Invalid(problem),
        None => HeaderStatus::Valid,
    }
}

fn find_header_problem(lines: &[&str]) -> Option<HeaderProblem> {
    match lines.first() {
        Some(first_line) if first_line.starts_with(SPDX_PREFIX) => {}
        _ => return Some(HeaderProblem::NotAtTop),
    }
    let copyright_line_count = count_leading_copyright_lines(lines);

    if copyright_line_count == 0 {
        return Some(HeaderProblem::NoCopyrightLine);
    }
    match lines
        .get(copyright_line_count)
        .and_then(|line| line.strip_prefix(LICENSE_PREFIX))
    {
        None => return Some(HeaderProblem::NoLicenseLine),
        Some(license_id) if license_id != LICENSE_ID => return Some(HeaderProblem::WrongLicense),
        Some(_) => {}
    }
    match lines.get(copyright_line_count + 1) {
        None => None,
        Some(line) if line.trim().is_empty() => None,
        Some(_) => Some(HeaderProblem::NoBlankLineAfter),
    }
}

fn count_leading_copyright_lines(lines: &[&str]) -> usize {
    let mut count = 0;

    for line in lines {
        match line.strip_prefix(COPYRIGHT_PREFIX) {
            Some(holder) if !holder.trim().is_empty() => count += 1,
            _ => break,
        }
    }
    count
}

fn prepend_default_header(content: &str) -> String {
    let header = default_header();

    if content.trim().is_empty() {
        header
    } else {
        format!("{header}\n{content}")
    }
}

fn default_header() -> String {
    format!("{COPYRIGHT_PREFIX}{DEFAULT_COPYRIGHT_HOLDER}\n{LICENSE_PREFIX}{LICENSE_ID}\n")
}

fn header_problem_message(problem: &HeaderProblem) -> String {
    match problem {
        HeaderProblem::StartsWithByteOrderMark => {
            "the file starts with a byte order mark (BOM). Remove it".to_string()
        }
        HeaderProblem::NotAtTop => "SPDX lines exist, but not on the first line".to_string(),
        HeaderProblem::NoCopyrightLine => "no SPDX-FileCopyrightText line at the top".to_string(),
        HeaderProblem::NoLicenseLine => {
            "no SPDX-License-Identifier line after the copyright line(s)".to_string()
        }
        HeaderProblem::WrongLicense => format!("the license identifier is not {LICENSE_ID}"),
        HeaderProblem::NoBlankLineAfter => "no blank line after the header".to_string(),
    }
}

fn print_header_problem(file: &Path, problem: &HeaderProblem) {
    println!(
        "invalid header: {}: {}",
        relative_to_workspace_root(file).display(),
        header_problem_message(problem)
    );
}

fn read_source_file(file: &Path) -> Option<String> {
    match fs::read_to_string(file) {
        Ok(content) => Some(content),
        Err(error) => {
            eprintln!(
                "could not read {}: {error}",
                relative_to_workspace_root(file).display()
            );
            None
        }
    }
}

fn collect_workspace_source_files() -> Option<Vec<PathBuf>> {
    let mut source_files = Vec::new();

    if let Err(error) = collect_source_files_recursively(workspace_root(), &mut source_files) {
        eprintln!("could not list the workspace files: {error}");
        return None;
    }
    source_files.sort();
    Some(source_files)
}

fn collect_source_files_recursively(
    directory: &Path,
    source_files: &mut Vec<PathBuf>,
) -> io::Result<()> {
    let entries = fs::read_dir(directory).map_err(|error| add_path_to_error(directory, error))?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            if !is_skipped_directory(&path) {
                collect_source_files_recursively(&path, source_files)?;
            }
        } else if file_type.is_file() && is_source_file(&path) {
            source_files.push(path);
        }
    }
    Ok(())
}

fn add_path_to_error(path: &Path, error: io::Error) -> io::Error {
    io::Error::new(error.kind(), format!("{}: {error}", path.display()))
}

fn is_skipped_directory(directory: &Path) -> bool {
    let is_build_output = directory == workspace_root().join(BUILD_OUTPUT_DIRECTORY);
    let is_hidden = match directory.file_name() {
        Some(name) => name.to_string_lossy().starts_with('.'),
        None => false,
    };
    is_build_output || is_hidden
}

fn is_source_file(file: &Path) -> bool {
    match file.extension().and_then(|extension| extension.to_str()) {
        Some(extension) => SOURCE_FILE_EXTENSIONS.contains(&extension),
        None => false,
    }
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the xtask crate should sit directly inside the workspace root")
}

fn relative_to_workspace_root(file: &Path) -> &Path {
    file.strip_prefix(workspace_root()).unwrap_or(file)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod classify_header {
        use super::*;

        #[test]
        fn two_line_header_is_valid() {
            let content = concat!(
                "// SPDX-FileCopyrightText: Johan Lager\n",
                "// SPDX-License-Identifier: GPL-3.0-only\n",
                "\n",
                "fn main() {}\n",
            );
            assert_eq!(classify_header(content), HeaderStatus::Valid);
        }

        #[test]
        fn two_copyright_lines_are_valid() {
            let content = concat!(
                "// SPDX-FileCopyrightText: Johan Lager\n",
                "// SPDX-FileCopyrightText: John Rust\n",
                "// SPDX-License-Identifier: GPL-3.0-only\n",
                "\n",
                "fn main() {}\n",
            );
            assert_eq!(classify_header(content), HeaderStatus::Valid);
        }

        #[test]
        fn header_then_end_of_file_is_valid() {
            let content = concat!(
                "// SPDX-FileCopyrightText: Johan Lager\n",
                "// SPDX-License-Identifier: GPL-3.0-only\n",
            );
            assert_eq!(classify_header(content), HeaderStatus::Valid);
        }

        #[test]
        fn no_header_is_missing() {
            let content = "//! Crate docs.\n\nfn main() {}\n";

            assert_eq!(classify_header(content), HeaderStatus::Missing);
        }

        #[test]
        fn wrong_license_is_invalid() {
            let content = concat!(
                "// SPDX-FileCopyrightText: Johan Lager\n",
                "// SPDX-License-Identifier: GPL-3.0-or-later\n",
                "\n",
                "fn main() {}\n",
            );
            assert_eq!(
                classify_header(content),
                HeaderStatus::Invalid(HeaderProblem::WrongLicense)
            );
        }

        #[test]
        fn code_right_after_header_is_invalid() {
            let content = concat!(
                "// SPDX-FileCopyrightText: Johan Lager\n",
                "// SPDX-License-Identifier: GPL-3.0-only\n",
                "fn main() {}\n",
            );
            assert_eq!(
                classify_header(content),
                HeaderStatus::Invalid(HeaderProblem::NoBlankLineAfter)
            );
        }

        #[test]
        fn blank_line_before_header_is_invalid() {
            let content = concat!(
                "\n",
                "// SPDX-FileCopyrightText: Johan Lager\n",
                "// SPDX-License-Identifier: GPL-3.0-only\n",
                "\n",
                "fn main() {}\n",
            );
            assert_eq!(
                classify_header(content),
                HeaderStatus::Invalid(HeaderProblem::NotAtTop)
            );
        }

        #[test]
        fn copyright_without_license_is_invalid() {
            let content = concat!(
                "// SPDX-FileCopyrightText: Johan Lager\n",
                "\n",
                "fn main() {}\n",
            );
            assert_eq!(
                classify_header(content),
                HeaderStatus::Invalid(HeaderProblem::NoLicenseLine)
            );
        }

        #[test]
        fn license_without_copyright_is_invalid() {
            let content = concat!(
                "// SPDX-License-Identifier: GPL-3.0-only\n",
                "\n",
                "fn main() {}\n",
            );
            assert_eq!(
                classify_header(content),
                HeaderStatus::Invalid(HeaderProblem::NoCopyrightLine)
            );
        }
    }

    mod prepend_default_header {
        use super::*;

        #[test]
        fn code_file_returns_header_blank_line_and_original_content() {
            let content = "fn main() {}\n";
            let expected = concat!(
                "// SPDX-FileCopyrightText: Johan Lager\n",
                "// SPDX-License-Identifier: GPL-3.0-only\n",
                "\n",
                "fn main() {}\n",
            );
            assert_eq!(prepend_default_header(content), expected);
        }

        #[test]
        fn empty_file_returns_header_only() {
            let expected = concat!(
                "// SPDX-FileCopyrightText: Johan Lager\n",
                "// SPDX-License-Identifier: GPL-3.0-only\n",
            );
            assert_eq!(prepend_default_header(""), expected);
        }
    }

    mod license_id {
        use super::*;

        #[test]
        fn matches_workspace_manifest() {
            let workspace_manifest = include_str!("../../Cargo.toml");

            assert!(workspace_manifest.contains(&format!("license = \"{LICENSE_ID}\"")));
        }
    }
}
