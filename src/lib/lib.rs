use rust_i18n::i18n;
i18n!();

pub use rust_i18n::t;
pub mod core;
pub mod tui;

/// create string
///
/// create a new `String` from a string literal.
#[macro_export]
macro_rules! s {
    ($s:expr) => {
        String::from($s)
    };
}

/// fail macro
///
/// same as `bail!` but without the explicit return
#[macro_export]
macro_rules! fail {
    ($msg:literal $(,)?) => {
        Err(eyre::eyre!($msg))
    };
    ($err:expr $(,)?) => {
        Err(eyre::eyre!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        Err(eyre::eyre!($fmt, $($arg)*))
    };
}
