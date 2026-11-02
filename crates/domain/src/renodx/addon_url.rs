// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(test)]
mod tests;

use crate::link::{LinkProblem, parse_https_url};
use std::fmt;
use url::Url;

/// The extension every 64-bit `RenoDX` addon uses.
pub const ADDON_64_EXTENSION: &str = ".addon64";

/// The extension every 32-bit `RenoDX` addon uses.
pub const ADDON_32_EXTENSION: &str = ".addon32";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Addon64Url(Url);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Addon32Url(Url);

impl Addon64Url {
    pub fn parse(text: &str) -> Result<Addon64Url, LinkProblem> {
        let url = parse_addon_url(text, ADDON_64_EXTENSION)?;

        Ok(Addon64Url(url))
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// The file name exactly as it appears in the URL, percent-encoded. Letter case kept.
    pub fn file_name(&self) -> &str {
        last_path_segment(&self.0)
    }
}

impl Addon32Url {
    pub fn parse(text: &str) -> Result<Addon32Url, LinkProblem> {
        let url = parse_addon_url(text, ADDON_32_EXTENSION)?;

        Ok(Addon32Url(url))
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// The file name exactly as it appears in the URL, percent-encoded. Letter case kept.
    pub fn file_name(&self) -> &str {
        last_path_segment(&self.0)
    }
}

impl fmt::Display for Addon64Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl fmt::Display for Addon32Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

fn parse_addon_url(text: &str, extension: &str) -> Result<Url, LinkProblem> {
    let url = parse_https_url(text)?;

    if url.query().is_some() || url.fragment().is_some() {
        return Err(LinkProblem::HasQueryOrFragment);
    }
    if !has_addon_file_name(&url, extension) {
        return Err(LinkProblem::WrongExtension);
    }
    Ok(url)
}

fn has_addon_file_name(url: &Url, extension: &str) -> bool {
    let file_name = last_path_segment(url);

    file_name.len() > extension.len() && file_name.to_ascii_lowercase().ends_with(extension)
}

fn last_path_segment(url: &Url) -> &str {
    match url.path_segments() {
        Some(mut segments) => segments.next_back().unwrap_or(""),
        None => "",
    }
}
