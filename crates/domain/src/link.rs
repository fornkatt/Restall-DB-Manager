// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(test)]
mod tests;

use std::fmt;
use url::{Host, Url};

const NEXUS_MODS_HOSTS: &[&str] = &["www.nexusmods.com", "nexusmods.com"];
const GAME_BANANA_HOSTS: &[&str] = &["www.gamebanana.com", "gamebanana.com"];
const DISCORD_HOSTS: &[&str] = &["discord.com", "discord.gg", "discordapp.com"];
const GITHUB_HOST: &str = "github.com";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkProblem {
    NotAValidUrl,
    NotHttps,
    HasCredentialsOrPort,
    InvalidHost,
    WrongHost,
    WrongPath,
    HasQueryOrFragment,
    WrongExtension,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NexusModsUrl(Url);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameBananaUrl(Url);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiscordUrl(Url);

/// A GitHub Discussions thread: `github.com/<owner>/<repository>/discussions/<number>`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiscussionUrl(Url);

impl NexusModsUrl {
    pub fn parse(text: &str) -> Result<NexusModsUrl, LinkProblem> {
        let url = parse_https_url(text)?;

        check_host(&url, NEXUS_MODS_HOSTS)?;

        Ok(NexusModsUrl(url))
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl GameBananaUrl {
    pub fn parse(text: &str) -> Result<GameBananaUrl, LinkProblem> {
        let url = parse_https_url(text)?;

        check_host(&url, GAME_BANANA_HOSTS)?;

        Ok(GameBananaUrl(url))
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl DiscordUrl {
    pub fn parse(text: &str) -> Result<DiscordUrl, LinkProblem> {
        let url = parse_https_url(text)?;

        check_host(&url, DISCORD_HOSTS)?;

        Ok(DiscordUrl(url))
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl DiscussionUrl {
    pub fn parse(text: &str) -> Result<DiscussionUrl, LinkProblem> {
        let url = parse_https_url(text)?;

        check_host(&url, &[GITHUB_HOST])?;

        if !is_discussion_path(&url) {
            return Err(LinkProblem::WrongPath);
        }
        Ok(DiscussionUrl(url))
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Compares owner and repository names case-insensitively.
    pub fn is_in_repository(&self, owner: &str, repository: &str) -> bool {
        match non_empty_path_segments(&self.0).as_slice() {
            [found_owner, found_repository, ..] => {
                found_owner.eq_ignore_ascii_case(owner)
                    && found_repository.eq_ignore_ascii_case(repository)
            }
            _ => false,
        }
    }
}

impl fmt::Display for NexusModsUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl fmt::Display for GameBananaUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl fmt::Display for DiscordUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl fmt::Display for DiscussionUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

pub(crate) fn parse_https_url(text: &str) -> Result<Url, LinkProblem> {
    #[expect(
        clippy::map_err_ignore,
        reason = "url::ParseError is a foreign type. To users it only means 'not a URL'"
    )]
    let url = Url::parse(text).map_err(|_| LinkProblem::NotAValidUrl)?;

    if url.scheme() != "https" {
        return Err(LinkProblem::NotHttps);
    }
    if !url.username().is_empty() || url.password().is_some() || url.port().is_some() {
        return Err(LinkProblem::HasCredentialsOrPort);
    }
    match url.host() {
        Some(Host::Domain(domain)) if is_multi_label_domain(domain) => Ok(url),
        _ => Err(LinkProblem::InvalidHost),
    }
}

fn is_multi_label_domain(domain: &str) -> bool {
    domain.contains('.') && domain.split('.').all(|label| !label.is_empty())
}

fn check_host(url: &Url, allowed_hosts: &[&str]) -> Result<(), LinkProblem> {
    match url.host_str() {
        Some(host) if allowed_hosts.contains(&host) => Ok(()),
        _ => Err(LinkProblem::WrongHost),
    }
}

fn is_discussion_path(url: &Url) -> bool {
    match non_empty_path_segments(url).as_slice() {
        [_, _, "discussions", number] => number.chars().all(|character| character.is_ascii_digit()),
        _ => false,
    }
}

fn non_empty_path_segments(url: &Url) -> Vec<&str> {
    match url.path_segments() {
        Some(segments) => segments.filter(|segment| !segment.is_empty()).collect(),
        None => vec![],
    }
}
