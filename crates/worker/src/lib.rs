// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

mod work;
mod worker;

pub use work::{OpenedGameMods, ResultSink, WorkResult};
pub use worker::{WorkRequests, Worker, WorkerThread};
