use std::error;
use std::fmt;

// #[derive(Debug, Clone)]
// pub struct ErrInfo {
//     code: u32,
//     msg: String,
// }

// impl ErrInfo {
//     pub fn new(code: u32, msg: String) -> ErrInfo {
//         ErrInfo {
//             code: code,
//             msg: msg,
//         }
//     }
// }

type Result<T> = std::result::Result<T, PyProjErr>;

// Attempt to create project at existing directory."
#[derive(Debug, Clone)]
pub enum PyProjErr {
    PyProjDirectoryExists(), //"Attempted to create project at existing directory.",
}

// TODO: Actually write good error messaging
impl fmt::Display for PyProjErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // println!("PyProjErr::fmt self = {:?}", self);
        write!(f, "my test error message")
    }
}

impl error::Error for PyProjErr {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
