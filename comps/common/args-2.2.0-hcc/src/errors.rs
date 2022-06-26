use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

/// An implementation of `Error` which may or may not include a scope and/or usage message.
pub struct ArgsError {
    desc: String,
}

impl ArgsError {
    /// Creates a new `ArgsError` with the provided `scope` and `msg`.
    /// If `scope` is an empty string (i.e. `""`) it will be ignored.
    pub fn new(scope: &str, msg: &str) -> ArgsError {
        Self::new_with_usage(scope, msg, "")
    }

    /// Creates a new `ArgsError` with the provided `scope`, `msg` and `usage` message.
    /// If either `scope` or `usage` are an empty string (i.e. `""`) they will be ignored.
    pub fn new_with_usage(scope: &str, msg: &str, usage: &str) -> ArgsError {
        use std::fmt::Write;
        // If there is a scope, append it to the front
        let mut v_desc = if scope.to_string().is_empty() {
            String::new()
        } else {
            format!("{}: ", scope)
        };

        // Append the error message
        v_desc.push_str(msg);

        // Append the usage message, if it exists
        if !usage.to_string().is_empty() {
            let _ = write!(v_desc, "\n\n{}", usage);
        }

        ArgsError { desc: v_desc }
    }
}

impl Debug for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl Display for ArgsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl Error for ArgsError {
    fn description(&self) -> &str {
        &self.desc
    }
}
