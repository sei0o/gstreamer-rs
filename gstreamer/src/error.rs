// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error;
use std::fmt;

use glib;
use glib::IsA;

#[macro_export]
macro_rules! gst_error_msg(
// Plain strings
    ($err:expr, ($msg:expr), [$dbg:expr]) =>  {
        $crate::ErrorMessage::new(&$err, Some($msg),
                          Some($dbg),
                          file!(), module_path!(), line!())
    };
    ($err:expr, ($msg:expr)) => {
        $crate::ErrorMessage::new(&$err, Some($msg),
                          None,
                          file!(), module_path!(), line!())
    };
    ($err:expr, [$dbg:expr]) => {
        $crate::ErrorMessage::new(&$err, None,
                          Some($dbg),
                          file!(), module_path!(), line!())
    };

// Format strings
    ($err:expr, ($($msg:tt)*), [$($dbg:tt)*]) =>  { {
        $crate::ErrorMessage::new(&$err, Some(format!($($msg)*).as_ref()),
                          Some(format!($($dbg)*).as_ref()),
                          file!(), module_path!(), line!())
    }};
    ($err:expr, ($($msg:tt)*)) =>  { {
        $crate::ErrorMessage::new(&$err, Some(format!($($msg)*).as_ref()),
                          None,
                          file!(), module_path!(), line!())
    }};

    ($err:expr, [$($dbg:tt)*]) =>  { {
        $crate::ErrorMessage::new(&$err, None,
                          Some(format!($($dbg)*).as_ref()),
                          file!(), module_path!(), line!())
    }};
);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorMessage {
    pub(crate) error_domain: glib::Quark,
    pub(crate) error_code: i32,
    pub(crate) message: Option<String>,
    pub(crate) debug: Option<String>,
    pub(crate) filename: &'static str,
    pub(crate) function: &'static str,
    pub(crate) line: u32,
}

impl ErrorMessage {
    pub fn new<T: ::MessageErrorDomain>(
        error: &T,
        message: Option<&str>,
        debug: Option<&str>,
        filename: &'static str,
        function: &'static str,
        line: u32,
    ) -> ErrorMessage {
        let error_domain = T::domain();
        let error_code = error.code();

        ErrorMessage {
            error_domain,
            error_code,
            message: message.map(String::from),
            debug: debug.map(String::from),
            filename,
            function,
            line,
        }
    }
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Error {:?} ({:?}) at {}:{}",
            self.message, self.debug, self.filename, self.line
        )
    }
}

impl Error for ErrorMessage {
    fn description(&self) -> &str {
        "ErrorMessage"
    }
}

#[macro_export]
macro_rules! gst_loggable_error(
// Plain strings
    ($cat:expr, $msg:expr) => {
        $crate::LoggableError::new($cat.clone(), glib_bool_error!($msg))
    };

// Format strings
    ($cat:expr, $($msg:tt)*) =>  { {
        $crate::LoggableError::new($cat.clone(), glib_bool_error!($($msg)*))
    }};
);

#[macro_export]
macro_rules! gst_result_from_gboolean(
// Plain strings
    ($gst_sys_bool:expr, $cat:expr, $msg:expr) =>  {
        $crate::glib::glib_result_from_gboolean!($gst_sys_bool, $msg)
            .map_err(|bool_err| $crate::LoggableError::new($cat.clone(), bool_err))
    };

// Format strings
    ($gst_sys_bool:expr, $cat:expr, $($msg:tt)*) =>  { {
        $crate::glib::glib_result_from_gboolean!($gst_sys_bool, $($msg)*)
            .map_err(|bool_err| $crate::LoggableError::new($cat.clone(), bool_err))
    }};
);

#[derive(Debug, Clone)]
pub struct LoggableError {
    category: ::DebugCategory,
    bool_error: glib::BoolError,
}

impl LoggableError {
    pub fn new(category: ::DebugCategory, bool_error: glib::BoolError) -> LoggableError {
        LoggableError {
            category,
            bool_error,
        }
    }

    pub fn log(&self) {
        self.category.log(
            None as Option<&::Object>,
            ::DebugLevel::Error,
            self.bool_error.filename,
            self.bool_error.function,
            self.bool_error.line,
            format_args!("{}", self.bool_error.message),
        );
    }

    pub fn log_with_object<O: IsA<::Object>>(&self, obj: &O) {
        self.category.log(
            Some(obj),
            ::DebugLevel::Error,
            self.bool_error.filename,
            self.bool_error.function,
            self.bool_error.line,
            format_args!("{}", self.bool_error.message),
        );
    }

    pub fn category(&self) -> ::DebugCategory {
        self.category
    }
}

impl From<glib::BoolError> for LoggableError {
    fn from(bool_error: glib::BoolError) -> Self {
        LoggableError {
            category: *::CAT_RUST,
            bool_error,
        }
    }
}

impl fmt::Display for LoggableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Error {:?}: {:?} at {}:{}",
            self.category.get_name(),
            self.bool_error.message,
            self.bool_error.filename,
            self.bool_error.line
        )
    }
}

impl Error for LoggableError {
    fn description(&self) -> &str {
        self.bool_error.message.as_ref()
    }
}
