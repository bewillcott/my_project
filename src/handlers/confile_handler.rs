//
// File Name:    confile_handler.rs
// Directory:    src/handlers
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
//! # ConfileHandler
//!

use std::{
    fmt,
    fs::File,
    io::{Error, ErrorKind::InvalidInput, Write},
};

use flogging::*;

///
/// Publishes log entries to the file whose name was provided during
/// initialization.
///
#[derive(Debug, Default)]
pub struct ConfileHandler {
    filename: String,
    con_fmt: Formatter,
    file_fmt: Formatter,
    file: Option<File>,
    writer: Option<Vec<u8>>,
}

impl ConfileHandler {
    fn create(filename: &str) -> Result<Self, Error> {
        if filename.is_empty() {
            return Err(Error::new(InvalidInput, "'filename' must not be empty"));
        }

        let fh = ConfileHandler {
            filename: filename.to_string(),
            con_fmt: FormatType::Simple.create(None),
            file_fmt: FormatType::Iso8601.create(None),
            file: {
                let f = File::options().append(true).create(true).open(filename)?;

                Some(f)
            },
            writer: None,
        };

        Ok(fh)
    }

    fn log(&self) -> String {
        if let Some(w) = self.writer.to_owned() {
            String::from_utf8(w).unwrap()
        } else {
            String::new()
        }
    }
}

impl fmt::Display for ConfileHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Console: {}\n{} : {}",
            self.con_fmt, self.filename, self.file_fmt
        )
    }
}

impl HandlerTrait for ConfileHandler {
    ///
    /// Create a new handler instance.
    ///
    /// ## Parameters
    /// - `name` - This the `filename` of the log file.
    ///
    fn create(name: &str) -> Result<Self, Error> {
        ConfileHandler::create(name)
    }

    ///
    /// Flushes and closes the file.\
    /// Also, removes the internal buffer, if in `test_mode`.\
    /// Will therefore, no longer be *in* `test_mode`.
    ///
    fn close(&mut self) {
        self.flush();
        self.file = None;
    }

    fn flush(&mut self) {
        if let Some(f) = &self.file {
            f.sync_all().expect("sync_all() failed");
        }
    }

    fn get_formatter(&self) -> Formatter {
        self.file_fmt.clone()
    }

    fn get_log(&self) -> String {
        self.log()
    }

    fn is_open(&self) -> bool {
        self.file.is_some()
    }

    fn publish(&mut self, log_entry: &LogEntry) {
        if self.is_open() {
            let mut buf = self.file_fmt.format(log_entry);
            buf.push('\n');

            if let Some(w) = self.writer.as_mut() {
                writeln!(w, "{}", self.con_fmt.format(log_entry)).expect("writeln!() failed");
                writeln!(w, "{}", self.file_fmt.format(log_entry)).expect("writeln!() failed");
            } else {
                println!("{}", self.con_fmt.format(log_entry));
                self.file.as_mut().unwrap().write_all(buf.as_bytes()).expect("writeln!() failed");
            }
        }
    }

    fn set_formatter(&mut self, formatter: Formatter) {
        self.file_fmt = formatter;
    }

    ///
    /// Sets the test mode to `state`.
    ///
    /// If set to `true`, use `get_log()` to obtain the
    /// log.
    ///
    fn set_test_mode(&mut self, state: bool) {
        if state {
            // true
            self.writer = Some(Vec::new());
        } else {
            self.writer = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::io::Read;

    #[test]
    fn confile_handler() {
        let mut log = Logger::custom_logger(
            module_path!(),
            "ConfileHandler",
            Box::new(ConfileHandler::create("test_logs/confile_handler.log").unwrap()),
        );

        log.set_fn_name("confile_handler");

        let h = log
            .get_handler(Handler::Custom("ConfileHandler".to_string()))
            .unwrap();

        h.set_test_mode(false);

        assert!(h.is_open());
        assert_eq!(
            h.get_formatter().to_string(),
            "dt_fmt: \"%+\" - fmt_string: \"{dt:35} {mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );

        log.info("trait methods");
        log.warning("The sky is falling!");

        let handler = log
            .get_handler(Handler::Custom("ConfileHandler".to_string()))
            .unwrap();

        assert_eq!(handler.get_log(), "".to_string());

        handler.flush();
        handler.close();
        log.exiting_with("This should get thrown away.");
    }

    #[test]
    fn confile_handler_file_test() {
        let re_str =
"^(?:\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}\\.\\d{9}\\+\\d{2}:\\d{2}) my_project::handlers::confile_handler::tests->confile_handler_file_test \\[INFO   ] trait methods
(?:\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}\\.\\d{9}\\+\\d{2}:\\d{2}) my_project::handlers::confile_handler::tests->confile_handler_file_test \\[WARNING] The sky is falling!
$";

        let re = Regex::new(re_str).unwrap();

        let mut log = Logger::builder(module_path!())
            .set_fn_name("confile_handler_file_test")
            .remove_file("test_logs/confile_handler_file_test.log")
            .add_custom_handler(
                "ConfileHandler",
                Box::new(
                    ConfileHandler::create("test_logs/confile_handler_file_test.log").unwrap(),
                ),
            )
            .build();

        let h = log
            .get_handler(Handler::Custom("ConfileHandler".to_string()))
            .unwrap();

        h.set_test_mode(false);

        assert!(h.is_open());
        assert_eq!(
            h.get_formatter().to_string(),
            "dt_fmt: \"%+\" - fmt_string: \"{dt:35} {mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );

        log.info("trait methods");
        log.warning("The sky is falling!");

        let h = log
            .get_handler(Handler::Custom("ConfileHandler".to_string()))
            .unwrap();

        assert_eq!(h.get_log(), "".to_string());

        h.flush();
        h.close();
        assert!(!h.is_open());

        log.severe("This should get thrown away.");

        if let Ok(mut file) = File::open("test_logs/confile_handler_file_test.log") {
            let mut buf = String::new();
            if let Ok(_count) = file.read_to_string(&mut buf) {
                assert!(re.is_match(&buf));
            }
        }
    }

    #[test]
    fn confile_handler_test_mode() {
        let re_str =
"^my_project::handlers::confile_handler::tests->confile_handler_test_mode \\[INFO   ] trait methods
(?:\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}\\.\\d{9}\\+\\d{2}:\\d{2}) my_project::handlers::confile_handler::tests->confile_handler_test_mode \\[INFO   ] trait methods
my_project::handlers::confile_handler::tests->confile_handler_test_mode \\[WARNING] The sky is falling!
(?:\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}\\.\\d{9}\\+\\d{2}:\\d{2}) my_project::handlers::confile_handler::tests->confile_handler_test_mode \\[WARNING] The sky is falling!
$";
        let re = Regex::new(re_str).unwrap();

        let mut log = Logger::builder(module_path!())
            .set_fn_name("confile_handler_test_mode")
            .add_custom_handler(
                "ConfileHandler",
                Box::new(
                    // This file is never written to:
                    ConfileHandler::create("test_logs/confile_handler_test_mode.log").unwrap(),
                ),
            )
            .build();

        let h = log
            .get_handler(Handler::Custom("ConfileHandler".to_string()))
            .unwrap();

        // All log entries will be stored in the internal buffer.
        h.set_test_mode(true);

        assert!(h.is_open());
        assert_eq!(
            h.get_formatter().to_string(),
            "dt_fmt: \"%+\" - fmt_string: \"{dt:35} {mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );

        log.info("trait methods");
        log.warning("The sky is falling!");

        let h = log
            .get_handler(Handler::Custom("ConfileHandler".to_string()))
            .unwrap();

        let buf = h.get_log();

        assert!(re.is_match(&buf));

        h.flush();
        h.close();
    }

    #[test]
    #[should_panic(expected = "'filename' must not be empty")]
    fn filename_empty() {
        let _ = Logger::builder(module_path!())
            .set_fn_name("confile_handler_test_mode")
            .add_custom_handler(
                "ConfileHandler",
                Box::new(
                    ConfileHandler::create("").unwrap(),
                ),
            )
            .build();
    }
}
