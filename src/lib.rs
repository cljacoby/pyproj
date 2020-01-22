pub mod cli;
pub mod err;

use err::PyProjErr;
// use err::{ErrInfo, PyProjErr};

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

// TODO: It seems like PyProj.path should be type Path and not PathBuf,
// but I can compile-time errors saying PyProf has unknown size at compile time.

// TODO: in PypProj.new, assess using many trait boundaries as opposed to just
// settling on using String

// TODO: Add ability to rename source directory something other than 'src'
// TODO: Assess if default src directory name should be 'src' or package name

// TODO: Intoducing builder pattern for PyProj. How should I handle path? Could
// initialzie it to a default location; however, this could result in directories
// unintentially being created. Setting as Option for now.

// TODO: Evaluate optimizing any excessive String allocations when handling
// string argument data between PyProj, PyProjBuilder, and PyProj.create()



#[derive(Debug)]
pub struct PyProj {
    name: String,
    path: PathBuf,
    src_name: String,
    with_script: bool,
    with_tests: bool,
    script_name: String,
    tests_name: String,
}

pub struct PyProjBuilder {
    name: String,
    path: Option<PathBuf>,
    src_name: String,
    with_script: bool,
    with_tests: bool,
    script_name: String,
    tests_name: String,
}

impl PyProjBuilder {
    pub fn new() -> Self {
        Self {
            name: String::from("unnamed_python_project"),
            path: None,
            src_name: String::from("src"),
            with_script: true,
            script_name: String::from("script"),
            tests_name: String::from("tests"),
            with_tests: true,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        Self {
            name: String::from(name),
            ..self
        }
    }

    pub fn with_path(mut self, path: PathBuf) -> Self {
        Self {
            path: Some(path),
            ..self
        }
    }

    pub fn with_src_name(mut self, src_name: &str) -> Self {
        Self {
            src_name: String::from(src_name),
            ..self
        }
    }
}

impl PyProj {
    pub fn new<S>(name: S, mut path: PathBuf) -> Self
    where
        S: Into<String>,
        S: std::convert::AsRef<std::path::Path>,
        S: Copy,
    {
        path.push(name);
        Self {
            name: name.into(),
            path: path,
            src_name: String::from("src"),
            with_script: true,
            with_tests: true,
            script_name: String::from("script"),
            tests_name: String::from("tests"),
        }
    }

    pub fn create(&self) -> Result<(), PyProjErr> {
        if self.path.exists() {
            return Result::Err(PyProjErr::App(()));
        }

        // Create project root
        fs::create_dir(self.path.as_path()).map_err(PyProjErr::Io)?;

        // Create project top level directories
        let mut src = self.path.clone();
        let mut dirs: Vec<&String> = Vec::new();
        dirs.push(&self.src_name);
        if self.with_script {
            dirs.push(&self.script_name);
        }
        if self.with_tests {
            dirs.push(&self.tests_name);
        }

        println!("begin dir loop");
        for path in dirs.iter() {
            src.push(path);
            fs::create_dir(src);
            println!("path = {:?}", src);
        }
        println!("end dir loop");

        //src.push(self.src_name.as_str());
        //fs::create_dir(src.as_path()).map_err(PyProjErr::Io)?;

        return Ok(());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create() {
        // Get UNIX seconds timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
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
