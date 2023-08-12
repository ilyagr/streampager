//! Command line definition for sp.

use clap::{Arg, Command};

pub(crate) fn app() -> Command {
    let app = Command::new("sp")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Stream Pager")
        .arg(
            Arg::new("FILE")
                .help("Displays the contents of this file")
                .multiple(true),
        )
        .arg(
            Arg::new("command")
                .long("command")
                .short('c')
                .value_name("\"COMMAND ARGS...\"")
                .help("Runs the command in a subshell and displays its output and error streams")
                .multiple(true),
        )
        .arg(
            Arg::new("fullscreen")
                .long("fullscreen")
                .short('F')
                .overrides_with_all(&["delayed", "no_alternate"])
                .help("Enter full screen immediately")
        )
        .arg(
            Arg::new("delayed")
                .long("delayed")
                .short('D')
                .value_name("SEC")
                .overrides_with_all(&["fullscreen", "no_alternate"])
                .help("Enter full screen after SEC seconds without waiting for content to fill one screen."),
        )
        .arg(
            Arg::new("no_alternate")
                .long("no-alternate")
                .short('X')
                .overrides_with_all(&["fullscreen", "delayed"])
                .help("Disables using the alternate screen. Enables streaming output before full screen."),
        );
    if cfg!(unix) {
        app.arg(
            Arg::new("fd")
                .long("fd")
                .value_name("FD[=TITLE]")
                .help("Displays the contents of this file descriptor")
                .multiple(true),
        )
        .arg(
            Arg::new("error_fd")
                .long("error-fd")
                .value_name("FD[=TITLE]")
                .help("Displays the contents of this file descriptor as the error stream of the previous file or file descriptor")
                .multiple(true),
        )
        .arg(
            Arg::new("progress_fd")
                .long("progress-fd")
                .value_name("FD")
                .help("Displays pages from this file descriptor as progress indicators"),
        )
    } else {
        app
    }
}
