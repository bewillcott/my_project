//
// File Name:    csv_formatter.rs
// Directory:    src/handlers/formatters
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
//! # CSV Formatter
//!

use std::fmt;
use flogging::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]

///
/// CSV format.
///
/// The datetime format string is:
///
/// ```text
/// "%Y-%m-%d %H:%M:%S%.6f"
/// ```
///
/// Example:
/// ```text
/// 2025-08-21 19:15:04.089061
/// ```
/// Template:
/// - `dt` in the template would be the datetime string, similar to the above.
/// - `mod_path`, `fn_name`, `level`, and `message` all come out of the `LogEntry`
///   provided to the [`format()`][CsvFormatter::format] method.
///
/// ```ignore
/// format!("{dt},{mod_path}->{fn_name},{level},\"{message}\"");
/// ```
/// Sample output:
/// ```text
/// 2025-08-21 19:26:47.801287,my_project::handlers::formatters::csv_formatter::tests->csv_format,SEVERE,"Hurricanes are windy!"
/// ```
///
pub struct CsvFormatter {
    dt_fmt: String,
    fmt_string: String,
}

impl CsvFormatter {
    ///
    /// Creates a new instance of `CsvFormatter`.
    ///
    pub fn new() -> Self {
        Self {
            dt_fmt: "%Y-%m-%d %H:%M:%S%.6f".to_string(),
            fmt_string: "{dt},{mod_path}->{fn_name},{level},\"{message}\"".to_string(),
        }
    }

    ///
    /// Returns the date/time format string.
    ///
    pub fn dt_fmt(&self) -> String {
        self.dt_fmt.clone()
    }

    ///
    /// Returns the primary format string.
    ///
    pub fn fmt_string(&self) -> String {
        self.fmt_string.clone()
    }
}

impl Default for CsvFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CsvFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "dt_fmt: \"{}\" - fmt_string: \"{}\"",
            self.dt_fmt, self.fmt_string
        )
    }
}

impl FormatTrait for CsvFormatter {
    fn format(&self, log_entry: &LogEntry) -> String {
        self.ft_fmt(self.dt_fmt(), self.fmt_string(), log_entry)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use regex::Regex;

    const_logger!({
        Logger::builder(module_path!())
            .add_string_handler_with(
                FormatType::Custom,
                Some(Box::new(CsvFormatter::new())),
            )
            .build()
    });

    #[test]
    #[logger]
    fn csv_format() {
        entering!();

        let re_str =
"^(?:\\d{4}-\\d{2}-\\d{2} \\d{2}:\\d{2}:\\d{2}\\.\\d{6}),my_project::handlers::formatters::csv_formatter::tests->csv_format,INFO,\"Testing a new custom formatter.\"
(?:\\d{4}-\\d{2}-\\d{2} \\d{2}:\\d{2}:\\d{2}\\.\\d{6}),my_project::handlers::formatters::csv_formatter::tests->csv_format,WARNING,\"Must add more testing.\"
$";

        let re = Regex::new(re_str).unwrap();

        info!("Testing a new custom formatter.");
        warning!("Must add more testing.");

        let log_str = get_handler!(Handler::String).unwrap().get_log();

        println!("{log_str}");
        assert!(re.is_match(&log_str));
    }
}
