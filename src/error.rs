//! Error types.

use std::ffi::OsStr;
use std::result::Result as StdResult;
use std::sync::mpsc::{RecvError, SendError, TryRecvError};

use thiserror::Error;

/// Convenient return type for functions.
pub type Result<T> = StdResult<T, Error>;

/// Main error type.
#[derive(Debug, Error)]
pub enum Error {
    /// Comes from [Termwiz](https://crates.io/crates/termwiz).
    #[error("terminal error")]
    Termwiz(#[source] anyhow::Error),

    /// Comes from [Regex](https://github.com/rust-lang/regex).
    #[error("regex error")]
    Regex(#[from] regex::Error),

    /// Generic I/O error.
    #[error("i/o error")]
    Io(#[from] std::io::Error),

    /// Returned when persisting a temporary file fails.
    #[error(transparent)]
    TempfilePersist(#[from] tempfile::PersistError),

    /// Keymap-related error.
    #[error("keymap error")]
    Keymap(#[from] crate::keymaps::error::KeymapError),

    /// Binding-related error.
    #[error("keybinding error")]
    Binding(#[from] crate::bindings::BindingError),

    /// Generic formatting error.
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),

    /// Receive error on a channel.
    #[error("channel error")]
    ChannelRecv(#[from] RecvError),

    /// Try-receive error on a channel.
    #[error("channel error")]
    ChannelTryRecv(#[from] TryRecvError),

    /// Send error on a channel.
    #[error("channel error")]
    ChannelSend,

    /// Error returned if the terminfo database is missing.
    #[error("terminfo database not found (is $TERM correct?)")]
    TerminfoDatabaseMissing,

    /// Wrapped error within the context of a command.
    #[error("error running command '{command}'")]
    WithCommand {
        /// Wrapped error.
        #[source]
        error: Box<Self>,

        /// Command the error is about.
        command: String,
    },

    /// Wrapped error within the context of a file.
    #[error("error loading file '{file}'")]
    WithFile {
        /// Wrapped error.
        #[source]
        error: Box<Self>,

        /// File the error is about.
        file: String,
    },
}

impl Error {
    pub(crate) fn with_file(self, file: impl AsRef<str>) -> Self {
        Self::WithFile {
            error: Box::new(self),
            file: file.as_ref().to_owned(),
        }
    }

    pub(crate) fn with_command(self, command: impl AsRef<OsStr>) -> Self {
        Self::WithCommand {
            error: Box::new(self),
            command: command.as_ref().to_string_lossy().to_string(),
        }
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_send_error: SendError<T>) -> Error {
        Error::ChannelSend
    }
}
