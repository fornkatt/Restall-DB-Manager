// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

#[derive(Debug, Clone, PartialEq, Eq)]
/// User-facing text for an action that didn't succeed.
pub struct Failure {
    pub summary: String,
    pub details: Vec<String>,
}
