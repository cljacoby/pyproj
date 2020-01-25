use std::error;
use std::fmt;
use std::io;

// NOTE: Guide on error handling patterns without a purpose-specific crate:
// https://blog.burntsushi.net/rust-error-handling/
// I've read through up to header 'The try! macro/? operator'

// TODO: What do I map Error::App to? I'm using this as an error I return
// when application-based errors are hit, rather than ones received from Results.
// Maybe define my own error type and map to that. Then the question is where to store
// the error description string.

// TODO: Is implementing Display and Error on the enum actually significant?
// I think maybe you're supposed to implement these on a user-defined error Struct,
// which would then be mapped to by an enum variant

#[derive(Debug)]
pub enum PyProjErr {
    App(String),
    Io(io::Error),
}

impl PyProjErr {
    pub fn app_err(msg: &str) -> PyProjErr {
        PyProjErr::App(msg.to_string())
    }
}

impl fmt::Display for PyProjErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            PyProjErr::Io(err) => format!("IO Error: {}", err),
            PyProjErr::App(err) => format!("PyProj Error: {}", err),
        };

        write!(f, "{}", msg)
    }
}

impl error::Error for PyProjErr {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<io::Error> for PyProjErr {
    fn from(err: io::Error) -> PyProjErr {
        PyProjErr::Io(err)
    }
}

impl From<PyProjErr> for String {
    fn from(err: PyProjErr) -> Self {
        format!("{}", err)
    }
}
