// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use super::*;

mod load_game_mods_error {
    use super::*;

    #[test]
    fn unreadable_returns_the_io_error_as_source() {
        let error = LoadGameModsError::Unreadable {
            path: PathBuf::from("game-mods.json"),
            source: io::Error::from(io::ErrorKind::PermissionDenied),
        };
        let source_kind = error
            .source()
            .and_then(|source| source.downcast_ref::<io::Error>())
            .map(io::Error::kind);

        assert_eq!(source_kind, Some(io::ErrorKind::PermissionDenied));
    }

    #[test]
    fn file_not_found_has_no_source() {
        let error = LoadGameModsError::FileNotFound {
            path: PathBuf::from("game-mods.json"),
        };

        assert!(error.source().is_none());
    }
}
