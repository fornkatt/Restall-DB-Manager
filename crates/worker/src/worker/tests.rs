// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use super::*;
use application::FileStamp;
use application::renodx::game_mods::{LoadGameModsError, LoadedGameMods};
use domain::component::ComponentNeeds;
use domain::id::ModId;
use domain::link::NexusModsUrl;
use domain::renodx::{GameMod, GameModParts, ModLinks, ModStatus};
use domain::text::{GameName, Maintainer};
use std::path::PathBuf;
use std::sync::mpsc::RecvTimeoutError;
use std::time::Duration;

mod open_game_mods {
    use super::*;

    #[test]
    fn readable_file_delivers_what_the_editor_opened() {
        let entries = vec![
            game_mod("control", "Control"),
            game_mod("control-ultimate-edition", "Control"),
        ];
        let editor = GameModsEditor::open(&FakeGameModsFile::containing(entries.clone())).unwrap();
        let (sink, delivered) = channel_sink();
        let worker = Worker::start(FakeGameModsFile::containing(entries), sink).unwrap();

        worker.requests.open_game_mods().unwrap();

        assert_eq!(
            delivered.recv_timeout(LONG_ENOUGH),
            Ok(WorkResult::OpenGameMods(Ok(OpenedGameMods {
                summaries: editor.summaries(),
                notices: editor.notices(),
            })))
        );
    }

    #[test]
    fn missing_file_delivers_the_failure_of_the_editor() {
        let failure = GameModsEditor::open(&FakeGameModsFile::missing()).unwrap_err();
        let (sink, delivered) = channel_sink();
        let worker = Worker::start(FakeGameModsFile::missing(), sink).unwrap();

        worker.requests.open_game_mods().unwrap();

        assert_eq!(
            delivered.recv_timeout(LONG_ENOUGH),
            Ok(WorkResult::OpenGameMods(Err(failure)))
        );
    }

    #[test]
    fn request_after_a_worker_panic_is_refused() {
        let (sink, delivered) = channel_sink();
        let Worker { requests, thread } =
            Worker::start(FakeGameModsFile::panicking(), sink).unwrap();

        requests.open_game_mods().unwrap();

        let nothing_delivered = delivered.recv_timeout(LONG_ENOUGH);
        let panicked = thread.finish();
        let refused = requests.open_game_mods();

        assert_eq!(nothing_delivered, Err(RecvTimeoutError::Disconnected));
        assert!(panicked.is_err(), "{panicked:?}");
        assert!(refused.is_err(), "{refused:?}");
    }
}

mod finish {
    use super::*;

    #[test]
    fn queued_request_is_answered_first() {
        let (sink, delivered) = channel_sink();
        let Worker { requests, thread } =
            Worker::start(FakeGameModsFile::containing(Vec::new()), sink).unwrap();

        requests.open_game_mods().unwrap();

        let finished = finished_in_time(thread);
        let answer = delivered.try_recv();

        assert_eq!(finished, Ok(true));
        assert!(answer.is_ok(), "{answer:?}");
    }

    #[test]
    fn returns_while_a_request_handle_is_still_alive() {
        let (sink, _delivered) = channel_sink();
        let Worker { requests, thread } =
            Worker::start(FakeGameModsFile::containing(Vec::new()), sink).unwrap();
        let finished = finished_in_time(thread);

        assert_eq!(finished, Ok(true));
        drop(requests);
    }
}

const LONG_ENOUGH: Duration = Duration::from_secs(10);

enum FakeLoad {
    Entries(Vec<GameMod>),
    Missing,
    Panic,
}

struct FakeGameModsFile {
    load: FakeLoad,
}

struct ChannelSink {
    results: Sender<WorkResult>,
}

impl FakeGameModsFile {
    fn containing(entries: Vec<GameMod>) -> FakeGameModsFile {
        FakeGameModsFile {
            load: FakeLoad::Entries(entries),
        }
    }
    fn missing() -> FakeGameModsFile {
        FakeGameModsFile {
            load: FakeLoad::Missing,
        }
    }
    fn panicking() -> FakeGameModsFile {
        FakeGameModsFile {
            load: FakeLoad::Panic,
        }
    }
}

impl LoadGameMods for FakeGameModsFile {
    fn load(&self) -> Result<LoadedGameMods, LoadGameModsError> {
        match &self.load {
            FakeLoad::Entries(entries) => Ok(LoadedGameMods {
                entries: entries.clone(),
                stamp: FileStamp::from_content(Vec::new()),
            }),
            FakeLoad::Missing => Err(LoadGameModsError::FileNotFound {
                path: PathBuf::from("game-mods.json"),
            }),
            FakeLoad::Panic => panic!("a bug on the worker thread"),
        }
    }
}

impl ResultSink for ChannelSink {
    fn deliver(&self, result: WorkResult) {
        self.results
            .send(result)
            .expect("the test should still be listening");
    }
}

fn finished_in_time(thread: WorkerThread) -> Result<bool, RecvTimeoutError> {
    let (outcomes, finished) = mpsc::channel();

    thread::spawn(move || {
        outcomes
            .send(thread.finish().is_ok())
            .expect("the test should still be listening");
    });

    finished.recv_timeout(LONG_ENOUGH)
}

fn channel_sink() -> (ChannelSink, Receiver<WorkResult>) {
    let (results, delivered) = mpsc::channel();

    (ChannelSink { results }, delivered)
}

fn game_mod(id: &str, name: &str) -> GameMod {
    GameMod::new(GameModParts {
        id: ModId::parse(id).unwrap(),
        name: GameName::parse(name).unwrap(),
        maintainer: Maintainer::parse("John Rust").unwrap(),
        status: ModStatus::Working,
        notes: None,
        links: ModLinks {
            snapshot_64: None,
            snapshot_32: None,
            nexus_mods: Some(
                NexusModsUrl::parse("https://www.nexusmods.com/control/mods/1").unwrap(),
            ),
            game_banana: None,
            discord: None,
            discussion: None,
        },
        depends: ComponentNeeds::default(),
        recommends: ComponentNeeds::default(),
    })
    .unwrap()
}
