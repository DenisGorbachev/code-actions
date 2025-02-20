use strum::Display;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum DiagnosticsProvider {
    ClippyJson,
    ClippyLib,
    RustcLib,
}

#[allow(dead_code)]
pub use DiagnosticsProvider::*;

impl DiagnosticsProvider {}
