//
// File Name:    control.rs
// Directory:    src/my_core
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
//! # Control mod
//!

use crate::*;

const_logger!({
    Logger::builder(module_path!())
        .add_console_handler()
        .add_file_handler("test_logs/usage.log")
        .set_level(DEBUG_LEVEL)
        //         ^^^^^^^^^^^
        .build()
});

#[logger]
pub fn do_it() {
    entering!();
    info!("Hello from `Control`.");
    exiting!();
}
