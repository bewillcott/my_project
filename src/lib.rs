//
// File Name:    lib.rs
// Directory:    src
// Project Name: my_project
//
// Copyright (C) 2025 Bradley Willcott
//
// SPDX-License-Identifier: GPL-3.0-or-later
//
// This library (crate) is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This library (crate) is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this library (crate).  If not, see <https://www.gnu.org/licenses/>.
//

//!
//! # My Project mod (Idea branch)
//!

mod my_core;

pub(crate) use flogging::*;
pub use my_core::*;

//
// Cargo.toml
//
// [dependencies]
// ctor = "0.5.0"
use ctor::*;

// pub(crate) const DEBUG_LEVEL: Level = Level::ALL;
pub(crate) const DEBUG_LEVEL:Level = Level::OFF;

///
/// Reset the log file each time `my_project` is loaded.
///
/// This is an alternative to using `remove_file()` in
/// the individual mod/file setup commands.\
/// Only useful if all child mods are using the same log file.
///
#[ctor]
fn reset_log() {
    Logger::remove_file("test_logs/usage.log");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control() {
        my_core::control::do_it();
    }

    #[test]
    fn my_core() {
        my_core::do_it();
    }
}
