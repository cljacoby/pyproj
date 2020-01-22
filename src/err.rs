use std::error;
use std::fmt;
use std::io;

// NOTE: Guide on error handling patterns without a purpose-specific crate:
// https://blog.burntsushi.net/rust-error-handling/
// I've read through up to header 'The try! macro/? operator'

type Result<T> = std::result::Result<T, PyProjErr>;

// TODO: What do I map PyProjErr::App to? I'm using this as an error I return
// when application-based errors are hit, rather than ones received from Results.
// Maybe define my own error type and map to that. Then the question is where to store
// the error description string.

#[derive(Debug)]
pub enum PyProjErr {
    App(()),
    // App(Io::Error),
    Io(io::Error),
}

// TODO: Is implementing Display and Error on the enum actually significant?
// I think maybe you're supposed to implement these on a user-defined error Struct,
// which would then be mapped to by an enum variant

impl fmt::Display for PyProjErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "my test error message")
    }
}

impl error::Error for PyProjErr {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
