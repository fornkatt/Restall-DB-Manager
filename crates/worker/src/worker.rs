// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use crate::work::WorkRequest;
use crate::{OpenedGameMods, ResultSink, WorkResult};
use application::Failure;
use application::renodx::game_mods::{GameModsEditor, LoadGameMods};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::{io, thread};

#[cfg(test)]
mod tests;

const THREAD_NAME: &str = "worker";

#[derive(Debug)]
pub struct Worker {
    pub requests: WorkRequests,
    pub thread: WorkerThread,
}

#[derive(Debug)]
pub struct WorkRequests {
    requests: Sender<WorkRequest>,
}

#[derive(Debug)]
pub struct WorkerThread {
    requests: Sender<WorkRequest>,
    thread: JoinHandle<()>,
}

impl Worker {
    pub fn start<F, S>(game_mods_file: F, sink: S) -> io::Result<Worker>
    where
        F: LoadGameMods + Send + 'static,
        S: ResultSink,
    {
        let (requests, pending_requests) = mpsc::channel();
        let stop_requests = requests.clone();
        let thread = thread::Builder::new()
            .name(THREAD_NAME.to_string())
            .spawn(move || serve_requests(&game_mods_file, &sink, &pending_requests))?;

        Ok(Worker {
            requests: WorkRequests { requests },
            thread: WorkerThread {
                requests: stop_requests,
                thread,
            },
        })
    }
}

impl WorkRequests {
    pub fn open_game_mods(&self) -> Result<(), Failure> {
        self.send(WorkRequest::OpenGameMods)
    }
    fn send(&self, request: WorkRequest) -> Result<(), Failure> {
        match self.requests.send(request) {
            Ok(()) => Ok(()),
            Err(_) => Err(worker_stopped_failure()),
        }
    }
}

impl WorkerThread {
    /// Lets the worker answer the requests it already has, then waits for it to end.
    pub fn finish(self) -> thread::Result<()> {
        // A failed send means the thread has already ended, and `join` reports why.
        let _ = self.requests.send(WorkRequest::Stop);

        self.thread.join()
    }
}

fn serve_requests<F: LoadGameMods, S: ResultSink>(
    game_mods_file: &F,
    sink: &S,
    pending_requests: &Receiver<WorkRequest>,
) {
    for request in pending_requests {
        match request {
            WorkRequest::OpenGameMods => {
                sink.deliver(WorkResult::OpenGameMods(opened_game_mods(game_mods_file)));
            }
            WorkRequest::Stop => return,
        }
    }
}

fn opened_game_mods<F: LoadGameMods>(game_mods_file: &F) -> Result<OpenedGameMods, Failure> {
    let editor = GameModsEditor::open(game_mods_file)?;

    Ok(OpenedGameMods {
        summaries: editor.summaries(),
        notices: editor.notices(),
    })
}

fn worker_stopped_failure() -> Failure {
    Failure {
        summary: "The background worker stopped unexpectedly.".to_string(),
        details: vec!["Restart the app.".to_string()],
    }
}
