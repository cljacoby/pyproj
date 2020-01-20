pub mod cli;
pub mod err;

use err::PyProjErr;
// use err::{ErrInfo, PyProjErr};

use std::fs;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH, Instant};

// TODO: It seems like PyProj.path should be type Path and not PathBuf,
// but I can compile-time errors saying PyProf has unknown size at compile time.

// TODO: in PypProj.new, assess using many trait boundaries as opposed to just
// settling on using String

// TODO: Add ability to rename source directory something other than 'src'
// TODO: Assess if default src directory name should be 'src' or package name

#[derive(Debug)]
pub struct PyProj {
    name: String,
    path: PathBuf,
    src_name: String,
}

impl PyProj {
    pub fn new<S>(name: S, mut path: PathBuf) -> PyProj
    where
        S: Into<String>,
        S: std::convert::AsRef<std::path::Path>,
        S: Copy,
    {
        path.push(name);
        PyProj {
            name: name.into(),
            path: path,
            src_name: String::from("src"),
        }
    }

    pub fn create(&self) -> Result<(), PyProjErr> {
        if self.path.exists() {
            return Result::Err(PyProjErr::PyProjDirectoryExists());
        }

        // Create project directory
        match fs::create_dir(self.path.as_path()) {
            Ok(val) => { },
            Err(err) => { return Result::Err(PyProjErr::PyProjDirectoryExists()); },
        }

        // Create source directory
        let mut src = self.path.clone();
        src.push(self.src_name.as_str());
        match fs::create_dir(src.as_path()) {
            Ok(val) => { },
            Err(err) => { return Result::Err(PyProjErr::PyProjCreateFailure()); },
        }


        return Ok(());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create() {
        
        // Get UNIX seconds timestamp
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("error: Timestamp calculation failed.")
            .as_secs()
            .to_string();
        // println!("timestamp = {:?}", timestamp);

        // create test directory
        let path = env::temp_dir();
        let name = format!("{}_{}", "test_pyproj", timestamp);
        let pyproj = PyProj::new(name.as_str(), path.clone());
        pyproj.create();
        
        // Remove after test
        let mut test_dir = path.clone();
        test_dir.push(name);
        fs::remove_dir_all(test_dir.as_path());

    }
}
