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

use my_project::*;

const_logger!({
    Logger::builder(module_path!())
        .remove_file("test_logs/debug.log")
        .add_custom_handler_with(
            "ConfileHandler",
            Box::new(ConfileHandler::create("test_logs/debug.log").unwrap()),
            FormatType::Custom,
            Some(Box::new(CsvFormatter::new())),
        )
        .set_level(Level::ALL)
        .build()
});

#[logger]
fn main() {
    entering!();

    config!("Operating system: Fedora Linux");
    config!("Version: 42");

    info!(
        "This is a test of the integration of the 'FLogging crate' and the custom handler and formatter."
    );

    println!("*** My Project ***");

    exiting!();
}
