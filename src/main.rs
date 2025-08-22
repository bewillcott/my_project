//
// File Name:    main.rs
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
//! # main file
//!

use flogging::*;
use std::{error::Error, result::Result};

fn do_something() {
    let mut log = Logger::econsole_logger(module_path!());
    log.set_fn_name("do_something")
    .set_level(Level::FINEST);

    log.entering();

    // do some work worth noting
    let result = "Just something to log.";
    log.info(&format!("Did some work here.\n  {result}"));

    // ...

    log.fine("Bit more detail.");

    if let Err(e) = error_prone() {
        log.warning(&format!("Error: {}", e));
    }

    log.exiting();
}

fn error_prone() -> Result<(), Box<dyn Error>> {
    let mut log = Logger::econsole_logger(module_path!());
    log.set_fn_name("error_prone")
    .set_level(Level::FINEST);

    log.entering();
    let rtn = Err(Box::from("Bad day!"));
    log.exiting_with(&format!("{rtn:?}"));
    rtn
}

fn main() {
    let mut log = Logger::pconsole_logger(module_path!());
    log.set_fn_name("main")
    // .set_level(Level::FINEST);
    .set_level(Level::FINEST);

    log.entering();
    log.info("All logging macros accept the same parameters as `std::format!(...)`");
    log.warning("Those same macros (info, etc.) MUST have atleast one parameter.");
    log.config("This is running on Fedora Linux 42.");
    do_something();
    log.info("Job's done.");
    log.exiting();
}
