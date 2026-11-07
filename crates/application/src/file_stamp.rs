// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use std::fmt;

/// What a file contained when it was last loaded or stored.
pub struct FileStamp {
    content: Vec<u8>,
}

impl FileStamp {
    pub fn from_content(content: Vec<u8>) -> FileStamp {
        FileStamp { content }
    }

    pub fn matches(&self, content: &[u8]) -> bool {
        self.content == content
    }
}

impl fmt::Debug for FileStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileStamp")
            .field("length", &self.content.len())
            .finish()
    }
}
