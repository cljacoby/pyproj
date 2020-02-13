use std::error;
use std::fmt;
use std::io;
use std::time;

use clap::Error;

// NOTE: Guide on error handling patterns without a purpose-specific crate:
// https://blog.burntsushi.net/rust-error-handling/
// I've read through up to header 'The try! macro/? operator'

#[derive(Debug)]
pub enum PyProjErr {
    App(String),
    Test(String),
    Io(io::Error),
    SystemTime(time::SystemTimeError),
}

impl PyProjErr {
    pub fn app_err(msg: &str) -> PyProjErr {
        PyProjErr::App(msg.to_string())
    }

    pub fn test_err(msg: &str) -> PyProjErr {
        PyProjErr::Test(msg.to_string())
    }
}

impl fmt::Display for PyProjErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            PyProjErr::Io(err) => format!("IO Error: {}", err),
            PyProjErr::App(err) => format!("PyProj Error: {}", err),
            PyProjErr::Test(err) => format!("PyProj Test Error: {}", err),
            PyProjErr::SystemTime(err) => format!("System Time Error: {}", err),
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

impl From<time::SystemTimeError> for PyProjErr {
    fn from(err: time::SystemTimeError) -> PyProjErr {
        PyProjErr::SystemTime(err)
    }
}
