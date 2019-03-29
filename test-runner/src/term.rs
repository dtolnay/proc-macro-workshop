use std::sync::{Mutex, MutexGuard, PoisonError};

use lazy_static::lazy_static;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream as Stream, WriteColor};

lazy_static! {
    static ref TERM: Mutex<Stream> = Mutex::new(Stream::stderr(ColorChoice::Auto));
}

pub fn lock() -> MutexGuard<'static, Stream> {
    TERM.lock().unwrap_or_else(PoisonError::into_inner)
}

pub fn bold() {
    let _ = lock().set_color(ColorSpec::new().set_bold(true));
}

pub fn color(color: Color) {
    let _ = lock().set_color(ColorSpec::new().set_fg(Some(color)));
}

pub fn bold_color(color: Color) {
    let _ = lock().set_color(ColorSpec::new().set_bold(true).set_fg(Some(color)));
}

pub fn reset() {
    let _ = lock().reset();
}

#[deny(unused_macros)]
macro_rules! print {
    ($($args:tt)*) => {{
        use std::io::Write as _;
        let _ = std::write!($crate::term::lock(), $($args)*);
    }};
}

#[deny(unused_macros)]
macro_rules! println {
    ($($args:tt)*) => {{
        use std::io::Write as _;
        let _ = std::writeln!($crate::term::lock(), $($args)*);
    }};
}
